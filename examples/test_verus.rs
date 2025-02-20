use builtin_macros::*;
use vstd::prelude::*;
use vstd::simple_pptr::PPtr;
verus! {

    spec fn uninterp_fn(x: u64) -> bool;

    fn pop_test(t: Vec<u64>)
        requires
            t.len() > 0,
            forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]),
    {
        let mut t = t;
        let x = t.pop().unwrap();
        assert(uninterp_fn(x));
        assert(forall|i: int| #![auto] 0 <= i < t.len() ==> uninterp_fn(t[i]));
    }

    // fn ng() -> (b: bool) 
    // ensures b
    //  {
    //     // Introduce unsoundness
    //     assume(false == true);
    //     // assume(true);
    //     // 1
    //     false
    // }
    fn main() {
        let (ptr, Tracked(perm)) = PPtr::new(
            5
                );
    }

}
