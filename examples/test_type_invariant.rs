use vstd::prelude::*;  // Import standard Verus utilities for specifications and proofs

verus! {

    // Define a simple struct `Cap` that encapsulates a single usize value.
    struct Cap(usize);

    // Define a type invariant (using #[verifier::type_invariant]) for the `Cap` type.
    // A type invariant is a logical condition that must always hold for all instances of this type.
    #[verifier::type_invariant]
    spec fn type_inv(x: Cap) -> bool { 
        x.0 < 3  // Type invariant ensures the value in `Cap` is always less than 3.
    }

    // Example function `add` attempting to modify a `Cap` instance.
    fn add(mut x: Cap) -> Cap {
        x.0 = 4;  // Violates the type invariant (`x.0` must remain less than 3).
        x         // Returning `x` here fails Verus verification due to breaking the type invariant.
    }
}

// Standard Rust entry point; the verification logic above does not run at runtime.
fn main() {}

