use vstd::prelude::verus;
use builtin::SpecOrd;
verus!{
    fn brea(mut i: usize, j: usize) {
        while i > 0 
        invariant i >= 0
        ensures i == 0 || j == 0
        {
            i = ( i - 1) / 2;       
            if j == 0 {
                break
            }
        }
        assert(j == 0 || i == 0);
    }
}
