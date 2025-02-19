use builtin_macros::*;
use vstd::prelude::*;
// use vstd::simple_pptr::PPtr;
use vstd::simple_pptr::PPtr;
verus! {
    fn ng() -> (b: bool) 
    ensures b
     {
        // assume(false == true);
        // assume(true);
        // 1
        !false
    }
    fn main() {

                let (ptr, Tracked(perm)) = PPtr::new(
            5
                );

    }
}
