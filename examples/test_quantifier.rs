use vstd::prelude::*;  // Standard prelude imports for Verus specification and proofs

verus!{

    // Specification-only functions (declared without implementation),
    // representing abstract concepts used purely in logical verification.
    spec fn a(i: nat) -> nat;                    // Abstract function `a`
    spec fn b(i: nat) -> nat;                    // Abstract function `b`
    spec fn c(i: nat, j: nat) -> bool;           // Abstract predicate `c`
    spec fn d(i: nat) -> nat;                    // Abstract function `d` (unused here, likely for generality)
    spec fn p(i: nat) -> nat;                    // Abstract indexing or permutation function `p`

    // Proof function demonstrating reasoning with quantifiers and triggers
    proof fn testq(len: nat) {
        // Assumption 1:
        // For all i within range, predicate c holds between elements from a(i) and a(p(i)).
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))));

        // Assumption 2:
        // For all i within range, a(i) equals b(i), and a(p(i)) equals b(p(i)).
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] a(p(i)) == b(p(i)) && a(i) == b(i));

        // Repetition of assumption 1 (harmless redundancy), reinforcing logical clarity.
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))));

        // Assertion 1:
        // From assumptions above, we logically conclude that for all i in range:
        // - c(a(i), a(p(i))) holds,
        // - equalities between a and b remain valid.
        assert(forall|i: nat| 0 <= i < len ==> #[trigger]
            c(a(i), a(p(i))) && a(p(i)) == b(p(i)) && a(i) == b(i)
        );

        // Assertion 2:
        // Extends the previous assertion, adding a logically equivalent condition c(b(i), b(p(i))),
        // due to established equalities between `a` and `b`.
        assert(forall|i: nat| 0 <= i < len ==> #[trigger]
            c(a(i), a(p(i))) && a(p(i)) == b(p(i)) && a(i) == b(i) && c(b(i), b(p(i)))
        );

        // Final Assertion:
        // Simplifies and isolates the derived predicate:
        // For every i, c holds for the elements b(i) and b(p(i)),
        // leveraging previous proven equalities and assumptions.
        assert(forall|i: nat| 0 <= i < len ==> #[trigger] c(b(i), b(p(i))));
    }
}
