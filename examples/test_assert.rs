use vstd::prelude::*;
use builtin::SpecOrd;

verus!{

    // ------------------------------------------------------------------
    // An *external* assertion function.  Because of the
    // `#[verifier(external_body)]` attribute, Verus ignores the body when
    // proving and assumes **only** the given specification (`ensures b`).
    // In other words, any call site must convince the verifier that
    // `b` is true *before* the call, otherwise verification will fail.
    // ------------------------------------------------------------------
    #[verifier(external_body)]
    fn assert_dyn(b: bool)
        ensures b,
    {
        // At runtime we still execute `assert!(b)`, but Verus does **not**
        // look at this line – it trusts the spec.  The body is kept only
        // so executable Rust code compiles and runs.
        assert!(b);
    }

    // ---------------------------------------------------------------
    // A *successful* use‑case: we first prove `x > 0`, then we may call
    // both the dynamic (spec‑only) and static (`assert`) assertions.
    // ---------------------------------------------------------------
    fn assert_test(x: usize) {
        assert_dyn(x > 0); // succeeds: verifier knows `x > 0` here
        assert(x > 0);     // regular Verus assert – also succeeds
    }

    // -----------------------------------------------------------------
    // A *failing* example: if `x <= 1` the `assert_dyn(false)` call is
    // reachable, forcing the verifier to prove `false`, which is
    // impossible.  Likewise, the unconditional `assert(false)` below is
    // an immediate proof obligation that cannot be satisfied.
    // -----------------------------------------------------------------
    fn assert_false(x: usize) {
        if x > 1 {
            // This branch tries to assert a false proposition.
            // Verification will fail because `false` cannot be proven.
            assert_dyn(false);
        }
        // Unconditional failure – the verifier rejects this function.
        assert(false);
    }
}

fn main() {}
