use vstd::prelude::*;   // Verus prelude: standard Verus definitions and macros
use builtin::SpecOrd;   // Built-in Verus trait for specifying ordering properties

verus!{

    // A simple example function `f` demonstrating the use of preconditions.
    //
    // Preconditions (`requires`) specify what must be true before the function can safely execute.
    fn f(a: f32, b: f32) 
        requires a == 0.0  // Precondition: Verifier ensures this function is called only if `a` equals 0.0
    {
        // Function body intentionally empty; serves as an example to demonstrate preconditions.
    }

    // Externally verified assertion function (`assert_dyn`).
    //
    // Annotated with #[verifier(external_body)], meaning:
    // - The function's body is externally trusted, so Verus does not verify it internally.
    // - Its specification (ensures clause) is assumed true during verification.
    #[verifier(external_body)]
    fn assert_dyn(b: bool)
        ensures
            b,  // Ensures `b` is always true after calling this function.
    {
        assert!(b);  // Actual runtime assertion for debugging purposes.
    }

    // Test function demonstrating a successful dynamic assertion.
    fn assert_test(x: usize) {
        assert_dyn(x > 0); // Calls the externally trusted assertion, requiring `x > 0`.
        assert(x > 0);     // Static Verus assertion; verification confirms this always holds after the dynamic assertion.
    }

    // Test function intentionally demonstrating assertion failures.
    fn assert_false(x: usize) {
        if x > 1 {
            // Dynamically asserts `false`, will fail at verification time if `x > 1` is possible.
            assert_dyn(false); 
        }
        // Static assertion that always fails verification, illustrating a guaranteed verification failure.
        assert(false);
    }
}

// Standard Rust main entry point (empty because logic is demonstrated in verification context).
fn main(){}
