use vstd::prelude::*;
use vstd::raw_ptr::PointsToRaw;

verus!{

/// Deallocate with the global allocator.
// #[cfg(feature = "alloc")]
#[verifier::external_body]
pub fn reallocate(
    p: *mut u8,
    size: usize,
    align: usize,
    new_size: usize,
    Tracked(pt): Tracked<PointsToRaw>,
    // Tracked(dealloc): Tracked<Dealloc>,
    ) 
{

}

}
