#![feature(sized_type_properties)] // Enables an unstable Rust nightly feature needed for `mem::size_of`

use vstd::prelude::verus; // Import standard Verus prelude utilities for verification
use core::mem;            // Provides memory utilities (e.g., size_of)

verus! {

    // Unsafe function intended to illustrate handling size-based logic for generic types.
    //
    // `unsafe` keyword explicitly marks code that may violate Rust’s memory safety guarantees,
    // indicating that the programmer manually ensures safety conditions.
    unsafe fn new_cap<T>(cap: usize) {
        // Check if type T has zero size at compile-time:
        // - Zero-sized types (ZSTs) require special handling, as they don't occupy memory.
        if mem::size_of::<T>() == 0 {
            // Special handling for zero-sized types (no memory allocation needed).
        } else {
            // Normal handling for non-zero-sized types (requires allocation logic).
        }
    }

}

fn main(){}
