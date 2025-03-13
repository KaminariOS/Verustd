use vstd::prelude::*;
use vstd::pervasive::runtime_assert;

use core::mem;
verus!{

    fn op(a: Option<usize>) -> usize {
        if let Some(num) = a {
            num
        } else 
        {
            runtime_assert(false);
            unreached()
        }
    }
}

fn main(){}
