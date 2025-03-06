use vstd::prelude::*;

// Later need to replace these imports with imports from alloc_verified and core_verifed
use core::marker::PhantomData;
use alloc::alloc::{Allocator, Global, Layout};

verus!{

// A verified version of usizeNoHighBit in core 
struct Cap(usize);

#[verifier::inline]
pub open spec fn usizeNoHighBit(x: usize) -> bool {
    x <= usize::MAX >> 1
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

struct RawVecInner<T>(T);

// Verus does not support default type param yet so no Allocator = Global
pub(crate) struct RawVec<T, A: Allocator> {
    inner: RawVecInner<A>,
    _marker: PhantomData<T>,
}

}

