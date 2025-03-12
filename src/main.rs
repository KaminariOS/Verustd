use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, *};

mod allocator;
// pub use allocator::reallocate;
// use verified_lib::add;
verus!{
// fn test(x: usize) -> (res: usize) 
//     requires x > 5,
//     ensures res > 1{
//         add(x)
//     }

#[verifier::external_body]
fn main() {
    println!("Hello, world!");
}

}
