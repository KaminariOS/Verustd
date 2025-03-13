use vstd::prelude::*;
use core::mem::align_of;
use vstd::prelude::Tracked;
use core::marker::PhantomData;
use vstd::raw_ptr::DeallocData;
use vstd::raw_ptr::Dealloc;
use vstd::simple_pptr::PPtr;
use vstd::raw_ptr::PointsToRaw;
use vstd::layout::is_power_2;
verus!{

fn main(){}
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

}

