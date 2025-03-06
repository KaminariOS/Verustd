#![feature(sized_type_properties)]
use vstd::prelude::verus;
use core::mem;
verus!{

unsafe fn new_cap<T>(cap: usize) {
    if mem::size_of::<T>() == 0 { } else {  } }
}

fn main(){}
