
use vstd::prelude::*;
verus!{
    // fn over(x: usize) -> usize {
    //     x + 1
    // }
    fn over1(x: usize) -> (res: usize) 
    requires x < 1
    ensures res as nat - 1 < 0
    {
        x
    }
}

fn main() {}
