use vstd::prelude::*;
use vstd::ptr::{PPtr, PointsTo};

// Later need to replace these imports with imports from alloc_verified and core_verifed
use core::marker::PhantomData;
use core::mem;
use alloc::alloc::{Allocator, Global, Layout};

verus!{

// A verified version of usizeNoHighBit in core 
struct Cap(usize);

#[verifier::inline]
pub open spec fn usizeNoHighBit(x: usize) -> bool {
    x as nat <= isize::MAX 
}

impl Cap {
    // This is only safe if the precondition is satisfied. 
    pub const fn new_verified(x: usize) -> Self 
        requires usizeNoHighBit(x)
    {
        Self(x)
    }

    #[verifier::type_invariant]
    spec fn type_inv(self) -> bool {
            usizeNoHighBit(self.0)
        }
}

// Unfortunely Verus does not support this at the moment 
// const ZERO_CAP: Cap = Cap::new_verified(usize::MAX);

const fn ZERO_CAP() -> Cap {
    Cap::new_verified(0)
}

fn new_cap<T>(cap: usize) -> Cap 
        requires usizeNoHighBit(cap)
{
    if mem::size_of::<T>() == 0 { ZERO_CAP() } else {  Cap::new_verified(cap)  }
}

/// A low-level utility for more ergonomically allocating, reallocating, and deallocating
/// a buffer of memory on the heap without having to worry about all the corner cases
/// involved. This type is excellent for building your own data structures like Vec and VecDeque.
/// In particular:
///
/// * Produces `Unique::dangling()` on zero-sized types.
/// * Produces `Unique::dangling()` on zero-length allocations.
/// * Avoids freeing `Unique::dangling()`.
/// * Catches all overflows in capacity computations (promotes them to "capacity overflow" panics).
/// * Guards against 32-bit systems allocating more than `isize::MAX` bytes.
/// * Guards against overflowing your length.
/// * Calls `handle_alloc_error` for fallible allocations.
/// * Contains a `ptr::Unique` and thus endows the user with all related benefits.
/// * Uses the excess returned from the allocator to use the largest available capacity.
///
/// This type does not in anyway inspect the memory that it manages. When dropped it *will*
/// free its memory, but it *won't* try to drop its contents. It is up to the user of `RawVec`
/// to handle the actual things *stored* inside of a `RawVec`.
///
/// Note that the excess of a zero-sized types is always infinite, so `capacity()` always returns
/// `usize::MAX`. This means that you need to be careful when round-tripping this type with a
/// `Box<[T]>`, since `capacity()` won't yield the length.


// Verus does not support default type param yet so no Allocator = Global
pub(crate) struct RawVec<T, A: Allocator> {
    inner: RawVecInner<A>,
    _marker: PhantomData<T>,
}

/// Like a `RawVec`, but only generic over the allocator, not the type.
///
/// As such, all the methods need the layout passed-in as a parameter.
///
/// Having this separation reduces the amount of code we need to monomorphize,
/// as most operations don't need the actual type, just its layout.
struct RawVecInner<A: Allocator> {
    ptr: PPtr<u8>,
    pt: PointsTo<u8>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: Cap,
    alloc: A,
}

}

