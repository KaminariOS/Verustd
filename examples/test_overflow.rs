use vstd::prelude::*;  // Import standard Verus utilities for verification

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

    // Proof function demonstrating numeric underflow behavior with naturals (`nat`).
    proof fn over_nat() {
        let a: usize = 0;

        // assert(a - 1 < 5);
        // casting -1 to nat is like NaN
        // assert(((a - 1) as nat) == 5); 
    }

    // Specification function (pure, verification-only).
    // Adds one to input, explicitly cast back to usize.
    // Used for illustrating equivalence in assertions.
    spec fn add_1(x: usize) -> usize {
        (x + 1) as usize
    }

    // Function testing the `add_1` spec function.
    fn test_add1(x: usize) {
        if x < 5 {
            // Within this branch (x < 5), explicitly verifies equality between
            // standard arithmetic and our specification function.
            assert(x + 1 == add_1(x));
        }
        // This assertion checks equivalence without conditional protection,
        // potentially causing verification concerns for very large inputs (overflow).
        assert(x + 1 == add_1(x));
    }
}

// Standard Rust main function (verification code is tested independently by Verus).
fn main() {}
