#![feature(strict_provenance)]
use vstd::prelude::*;
use vstd::prelude::Tracked;
use vstd::raw_ptr::*;
use std::marker::PhantomData;

extern crate alloc;
verus!{
// This is one way to implement proof_of_work
pub tracked struct DeallocUsed {

}

impl DeallocUsed {
    pub spec fn provenance(self) -> Provenance;
}

#[verifier::external_body]
pub fn deallocate(
    p: *mut u8,
    size: usize,
    align: usize,
    Tracked(pt): Tracked<PointsToRaw>,
    Tracked(dealloc): Tracked<Dealloc>,
) -> (pow: Tracked<DeallocUsed>)
    requires
        dealloc.addr() == p.addr(),
        dealloc.size() == size,
        dealloc.align() == align,
        dealloc.provenance() == pt.provenance(),
        pt.is_range(dealloc.addr() as int, dealloc.size() as int),
        p@.provenance == dealloc.provenance(),
    ensures pow@.provenance() == dealloc.provenance()
    opens_invariants none
{
    // SAFETY: ensured by dealloc token
    let layout = unsafe { alloc::alloc::Layout::from_size_align_unchecked(size, align) };
    unsafe {
        alloc::alloc::dealloc(p, layout);
    }
    Tracked::assume_new()
}


// Another way is to make Dealloc has type state 
pub tracked struct DeallocTypeState<T> {
    p: PhantomData<T>
}

// enum DeallocState {
pub struct Dealloced;
pub struct ToBeDealloced;
// }

#[verifier::external_body]
pub fn deallocate_type_state(de: Tracked<DeallocTypeState<ToBeDealloced>>) 
        -> Tracked<DeallocTypeState<Dealloced>> {
    Tracked::assume_new()
}

}
