use vstd::prelude::*;
use builtin::SpecOrd;
verus!{

     #[verifier(external_body)]
    fn assert_dyn(b: bool)
        ensures
            b,
    {
        assert!(b);
    }

    fn assert_test(x: usize) {
        assert_dyn(x > 0);
        assert(x > 0);
    }
    fn assert_false(x: usize) {
        if (x > 1)
        {
            assert_dyn(false);
        }
        assert(false);
    }
}

fn main(){}
