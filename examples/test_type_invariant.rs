use vstd::prelude::*;
verus!{
struct Cap(usize);
#[verifier::type_invariant]
spec fn type_inv(x: Cap) -> bool { 
        x.0 < 3
    }

    fn add(mut x: Cap) -> Cap {
        x.0 = 4;
        x
    }
}
    fn main() {}
