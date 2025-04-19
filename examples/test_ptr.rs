use vstd::prelude::*;
use core::mem::align_of;
use vstd::prelude::Tracked;
use core::marker::PhantomData;
use vstd::raw_ptr::*;
use vstd::simple_pptr::PPtr;
use vstd::layout::is_power_2;
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

fn ptr_play<V>(val: V)  requires core::mem::size_of::<V>() != 0, core::mem::size_of::<V>() <= usize::MAX {
    // A new PPtr type that allows you to have PointsTo to different fields of a struct 
    // What is the benefit of it compared to an option? No need to move the object, all ghost.
    // I was wrong, PointsToRaw can be used to reason about arrays
    layout_for_type_is_valid::<V>();
    let size = core::mem::size_of::<V>();
    let align = core::mem::align_of::<V>();
    assume(2 * size <= isize::MAX as int - (isize::MAX as int % align as int));
    let (p, Tracked(points_to_raw), Tracked(dealloc)) = allocate(
                2 * core::mem::size_of::<V>(),
                core::mem::align_of::<V>(),
            );
    let tracked mut c;
    proof {
        let item_range = set_lib::set_int_range( p as int + size,
            p as int + 2 * size,
        );
        let tracked (mut a, b) = points_to_raw.split(item_range);
        c = a;
    }
    assume( p as usize <= usize::MAX );
    assume( p as usize + size <= usize::MAX );
    let provenance = expose_provenance(p);
    let new_p: *mut V = with_exposed_provenance(p as usize + size, provenance);
    assume(p as int == p as usize);
    assume(new_p as int == new_p as usize);
    assume(size % align == 0);
    assert(p as usize as int % align as int == 0);
    // assert((p as usize  as int + size) % align as int == 0)by (nonlinear_arith);
    // assert((p as usize + size) as int  % align as int == 0);
    assume(new_p as usize as int % align as int == 0) ;
    // assert(new_p as usize as int % align as int == 0) by (nonlinear_arith);
    let tracked mut c = c.into_typed::<V>((new_p as usize) as usize);
    unsafe {
    // How to get *mut V pointing to the second element?
        ptr_mut_write(new_p , Tracked(&mut c), val);
    }
}

    // proof fn address(addr: usize, size: usize, alignment: usize) 
    // requires alignment > 0, size % alignment == 0, addr % alignment == 0, addr < 1000000, size < 1000000, 
    // ensures (addr + size ) % (alignment as int) == 0
    // {
    //     broadcast use vstd::arithmetic::div_mod::lemma_mod_adds;
    //     assert ((addr + size ) % (alignment as int) == 0) by (nonlinear_arith);    
    // }


    proof fn address(addr: usize, size: usize, alignment: usize)
        requires
            alignment > 0,
            size % alignment == 0,
            addr % alignment == 0,
        ensures
            (addr + size) % (alignment as int) == 0,
    {
        broadcast use vstd::arithmetic::div_mod::lemma_mod_adds;
        // vstd::arithmetic::div_mod::lemma_mod_adds(addr as int, size as int, alignment as int);
    }
}

