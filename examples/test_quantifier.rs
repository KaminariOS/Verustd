use vstd::prelude::*;
verus!{
    spec fn a(i: nat) -> nat;
    spec fn b(i: nat) -> nat;
    spec fn c(i: nat, j: nat) -> bool;
    spec fn d(i: nat) -> nat;
    spec fn p(i: nat) -> nat;
    proof fn testq(len: nat) {
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))));
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] a(p(i)) == b(p(i)) && a(i) == b(i));
        assume(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))));
        assert(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))) && a(p(i)) == b(p(i)) && a(i) == b(i));
        assert(forall|i: nat| 0 <= i < len ==> #[trigger] c(a(i), a(p(i))) && a(p(i)) == b(p(i)) && a(i) == b(i) && c(b(i), b(p(i))));
        assert(forall|i: nat| 0 <= i < len ==>  #[trigger] c(b(i), b(p(i))));
    }
}
