
use vstd::prelude::*;
verus!{
    // fn over(x: usize) -> usize {
    //     x + 1
    // }
    // fn over1(x: usize) -> (res: usize) 
    // requires x < 1
    // ensures res as nat - 1 < 0
    // {
    //     x
    // }
    proof fn over_nat() {
        let a: usize = 0;
        // assert(a - 1 < 5);
        // casting -1 to nat is like NaN
        // assert(((a - 1) as nat) == 5); 
    }
    spec fn add_1(x: usize) -> usize {
        (x + 1) as usize
    }

    fn test_add1(x: usize){
        if x < 5 {
            assert(x + 1 == add_1(x));
        }
        assert(x + 1 == add_1(x));
    }
}

fn main() {}
