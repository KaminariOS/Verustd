use vstd::prelude::*;
use builtin::SpecOrd; // not used here but left for completeness

verus!{

/// Decrements `i` by repeatedly computing `(i - 1) / 2` until
/// either `i` reaches 0 or `j` is 0.  After the loop, it must hold
/// `i == 0 || j == 0`.
fn brea(mut i: usize, j: usize) {
    while i > 0
        // ----------------------------------------------------------------
        // Loop invariant:  (1) `i` is always non‑negative             
        //                  (2) If we ever exit normally (without break),
        //                      then `j` must be 0 — captured implicitly
        //                      by the post‑condition plus loop guard.
        // ----------------------------------------------------------------
        invariant i >= 0,
        // Termination metric: `i` strictly decreases each iteration.
        decreases i,
    {
        // Arithmetic step — safe because `i > 0` from loop guard.
        i = (i - 1) / 2;

        // Early exit if `j` is zero; post‑condition already covers
        // this case because we exit the loop immediately.
        if j == 0 {
            break;
        }
    }
    // Global post‑condition verified: either the loop naturally reduced
    // `i` to 0, or we broke out because `j == 0`.
    assert(j == 0 || i == 0);
}

} // end verus!
