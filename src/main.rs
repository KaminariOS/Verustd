use builtin::*;
use builtin_macros::*;
use vstd::{prelude::*, *};

mod allocator;
pub use allocator::reallocate;
verus!{

#[verifier::external_body]
fn main() {
    println!("Hello, world!");
}

}
