use vstd::prelude::*;

verus!{

    // ---------------------------------------------------------------------------
    // Total division with a pre‑condition and post‑condition
    // ---------------------------------------------------------------------------
    
    fn div(x: usize, y: usize) -> (r: usize)
        requires
            y > 0,            // prevent divide‑by‑zero
        ensures
            r * y == x,       // simple post‑condition for proof
    {
        x / y
    }
    
    // ---------------------------------------------------------------------------
    // 1. Success case ‑‑ should verify
    // ---------------------------------------------------------------------------
    
    fn test_div_ok() {
        let res = div(10, 2);
        assert(res == 5);
    }
    
    // ---------------------------------------------------------------------------
    // 2. Fail case #1: violates the pre‑condition (y == 0)
    // ---------------------------------------------------------------------------
    
    #[verifier::expect_fail]       // Tell Verus we *expect* this to fail
    fn test_div_fail_precondition() {
        // This call breaks the pre‑condition `y > 0`,
        // so verification should (and will) fail here.
        let _bad = div(10, 0);
    }
    
    // ---------------------------------------------------------------------------
    // 3. Fail case #2: wrong assertion (post‑condition not satisfied)
    // ---------------------------------------------------------------------------
    
    #[verifier::expect_fail]
    fn test_div_fail_postcondition() {
        let res = div(10, 2);
        // Deliberately wrong assertion – verification fails.
        assert(res == 6);
    }

} // end verus! block


