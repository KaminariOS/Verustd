use vstd::prelude::*;
verus!{

struct Cap(usize);

#[verifier::inline]
pub open spec fn usizeNoHighBit(x: usize) -> bool {
    x <= usize::MAX >> 1
}

impl Cap {
    pub fn new(x: usize) -> Self 
        requires usizeNoHighBit(x)
    {
        Self(x)
    }

    #[verifier::type_invariant]
    spec fn type_inv(self) -> bool {
            usizeNoHighBit(self.0)
        }
}


}
