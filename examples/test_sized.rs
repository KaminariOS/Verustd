#![feature(sized_type_properties)]
use core::mem::SizedTypeProperties;
use vstd::prelude::*;

verus!{
fn do_something_with<T>() {
    if T::IS_ZST {
        // ... special approach ...
    } else {
        // ... the normal thing ...
    }
}
}
