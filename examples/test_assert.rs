use vstd::prelude::verus;
use builtin::SpecOrd;
verus!{
    fn assert_test(x: usize) {
        assert!(x > 0);
        assert(x > 0);
    }
}

fn main(){}
