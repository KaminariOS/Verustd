use vstd::prelude::*;
use core::mem::align_of;
use vstd::prelude::Tracked;
use core::marker::PhantomData;
use vstd::raw_ptr::*;
use vstd::simple_pptr::PPtr;
use vstd::layout::*;
use vstd::layout::layout_for_type_is_valid;
use vstd::set_lib;
use vstd::arithmetic::div_mod::*;
verus!{

struct MyStruct {
    a: i32,
    b: i32,
}

fn main() {
    let mut s = MyStruct { a: 10, b: 20 };
    // let a_ref = (&mut s.a) as *mut i32;
    // let b_ref = &mut s.b;
    // *a_ref += 1;
    // *b_ref += 2;
    // println!("a: {}, b: {}", a_ref, b_ref);
}
#[verifier::external_body]
pub fn dangle<T>() -> (pt: (PPtr<T>, Tracked<PointsToRaw>, Tracked<Dealloc>)) 
    ensures 
        pt.1@.is_range(pt.0.addr() as int, 0 as int),
        pt.2@@ == (DeallocData {
            addr: pt.0.addr(),
            size: 0 as nat,
            align: align_of::<T>() as nat,
            provenance: pt.1@.provenance(),
        }),
        pt.0.addr() as int == align_of::<T>() as int,
        // pt.0@.metadata == Metadata::Thin,
        // pt.0@.provenance == pt.1@.provenance(),
    opens_invariants none
    {
    let pptr = PPtr(align_of::<T>(), PhantomData);
    (pptr, Tracked::assume_new(), Tracked::assume_new())
}

#[verifier::external_body]
pub fn dangle_aligned(align: usize) -> (pt: (PPtr<u8>, Tracked<PointsToRaw>, Tracked<Dealloc>)) 
    requires is_power_2(align as int)
    ensures 
        pt.1@.is_range(pt.0.addr() as int, 0 as int),
        pt.2@@ == (DeallocData {
            addr: pt.0.addr(),
            size: 0 as nat,
            align: align as nat,
            provenance: pt.1@.provenance(),
        }),
        pt.0.addr() as int == align as int,
        // pt.0@.metadata == Metadata::Thin,
        // pt.0@.provenance == pt.1@.provenance(),
    opens_invariants none
    {
    let pptr = PPtr(align, PhantomData); 
    (pptr, Tracked::assume_new(), Tracked::assume_new())
}

spec fn size_limit_for_valid_layout<V>(num: nat) -> bool {
    num * core::mem::size_of::<V>() <= isize::MAX as int - (isize::MAX as int % core::mem::align_of::<V>() as int)        
}

proof fn address_add_align(addr: usize, size: usize, alignment: usize)
    requires
        alignment > 0,
        size % alignment == 0,
        addr % alignment == 0,
    ensures
        (addr + size) % (alignment as int) == 0,
{
    broadcast use lemma_mod_adds;
    // vstd::arithmetic::div_mod::lemma_mod_adds(addr as int, size as int, alignment as int);
}

// A pedagogical example showcasing the usage of PointsToRaw ghost permission
fn write_to_raw_array<V>(first: V, second: V)  requires 
                    core::mem::size_of::<V>() != 0, 
                    size_limit_for_valid_layout::<V>(2)
{
    // This is an assumption that means 
    // we trust the Rust compiler for the layout of type V to be valid
    layout_for_type_is_valid::<V>();
    let size = core::mem::size_of::<V>();
    let align = core::mem::align_of::<V>();
    // p is the actual pointer returned by the allocator while points_to_raw and dealloc are ghost.
    // points_to_raw represents the permission to access the memory region pointed to by p 
    let (p, Tracked(points_to_raw), Tracked(dealloc)) = allocate(
                2 * size,
                align,
            );

    assume(p as usize + size <= usize::MAX);

    let tracked mut pointsToFirst;
    let tracked mut pointsToSecond;
    proof {
        // This should be included in the post condition of layout_for_type_is_valid
        assume(size % align == 0);
        let item_range = set_lib::set_int_range( p as int + size,
            p as int + 2 * size,
        );
        // We can split and merge memory permissions 
        let tracked (a, b) = points_to_raw.split(item_range);
        pointsToFirst = b;
        pointsToSecond = a;

        address_add_align(p as usize, size, align);
    }
    
    let tracked mut pointsToFirst = pointsToFirst.into_typed::<V>((p as usize) as usize);
    ptr_mut_write(p as *mut V, Tracked(&mut pointsToFirst), first);
    let provenance = expose_provenance(p);
    let new_p: *mut V = with_exposed_provenance(p as usize + size, provenance);
    let tracked mut pointsToSecond = pointsToSecond.into_typed::<V>((new_p as usize) as usize);
    ptr_mut_write(new_p , Tracked(&mut pointsToSecond), second);
}

    // proof fn address(addr: usize, size: usize, alignment: usize) 
    // requires alignment > 0, size % alignment == 0, addr % alignment == 0, addr < 1000000, size < 1000000, 
    // ensures (addr + size ) % (alignment as int) == 0
    // {
    //     broadcast use vstd::arithmetic::div_mod::lemma_mod_adds;
    //     assert ((addr + size ) % (alignment as int) == 0) by (nonlinear_arith);    
    // }



    

    fn test_allocate<V>() {
        layout_for_type_is_valid::<V>();
        let size = core::mem::size_of::<V>(); 
        let align = core::mem::align_of::<V>();
        assume(2 * size < isize::MAX as int - (isize::MAX as int % align as int));
        if  size != 0 {
                    assert(valid_layout((2 * size) as usize, align));
                    let (p, Tracked(points_to_raw), Tracked(dealloc)) = allocate(
                        2 * size,
                        align,
                    );

                    let (p1, Tracked(points_to_raw_1), Tracked(dealloc_1)) = allocate(
                        2 * size,
                        align,
                    );
                    // assert(p != p1);
        }
    }
}

