use vstd::prelude::*;
use vstd::modes::tracked_swap;

verus!{

// ─────────────────────────────────────────────────────────────
// MapWrapper — ghost map that stores WriteToken permissions by index
// ─────────────────────────────────────────────────────────────

tracked struct MapWrapper {
    inner_map: Map<nat, WriteToken>, // ghost map: index ↦ WriteToken
}

// A "View" gives an abstract, purely‑spec perspective of a tracked struct
impl View for MapWrapper {
    type V = Map<nat, WriteToken>;
    closed spec fn view(&self) -> Self::V { self.inner_map }
}

impl MapWrapper {
    /// Construct an empty permission map (all indices unallocated)
    proof fn empty() -> Self {
        Self { inner_map: Map::tracked_empty() }
    }

    /// Allocate a fresh WriteToken and insert it at the end of the map
    /// (implementation omitted — sketch shows intended spec guarantees)
    proof fn allocate(tracked &mut self) {
        // Intuitively: create WriteToken::fresh() and store it at key = len
    }

    /// Deallocate logic not yet implemented
    fn deallocate(&mut self) { }

    /// Pure spec: whether index `i` currently holds a *writable* token
    spec fn can_write(&self, i: nat) -> bool { self@[i].well_formed() }

    /// Map contains exactly the range 0‥len
    spec fn well_formed(&self) -> bool {
        &&& (forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i))
        &&& (forall|i: nat| i >= self@.len() ==> !self@.dom().contains(i))
    }

    /// Stronger: every entry also satisfies WriteToken::well_formed
    spec fn well_formed_and_complete(&self) -> bool {
        forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i) && #[trigger] self@[i].well_formed()
    }
}

// ─────────────────────────────────────────────────────────────
// Token types
//   • KeepToken   — never leaves the writer (private capability)
//   • GiveoutToken— may be handed to a reader; carries same id()
//   • WriteToken  — pair of KeepToken + optional GiveoutToken
// ─────────────────────────────────────────────────────────────

tracked struct KeepToken;
impl KeepToken { spec fn id(&self) -> int; }

tracked struct GiveoutToken;
impl GiveoutToken { spec fn id(&self) -> int; }

tracked struct WriteToken {
    keep:   KeepToken,
    giveout: Option<GiveoutToken>, // Some => writer currently *cannot* mutate
}

impl WriteToken {
    /// *Type invariant*: if giveout exists it shares the same id as keep
    #[verifier::type_invariant]
    spec fn type_inv(&self) -> bool {
        if let Some(g) = self.giveout { g.id() == self.keep.id() } else { true }
    }

    /// Token is *well‑formed* iff giveout is present (writer waiting for it back)
    spec fn well_formed(&self) -> bool { self.giveout.is_some() }

    /// Split: writer hands out the GiveoutToken and loses mutation rights
    proof fn split(tracked &mut self) -> (tracked res: GiveoutToken)
        requires old(self).well_formed()
        ensures  !self.well_formed(),
                 res == old(self).giveout.get_Some_0(),
                 res.id() == self.keep.id(),
                 res.id() == old(self).keep.id()
    {
        use_type_invariant(&*self);
        let tracked mut tmp = None;          // temporary Option<GiveoutToken>
        tracked_swap(&mut self.giveout, &mut tmp); // move giveout out
        tmp.tracked_unwrap() // return the token
    }

    /// Merge: writer regains mutation rights by consuming GiveoutToken
    proof fn merge(tracked &mut self, tracked g: GiveoutToken)
        requires !old(self).well_formed(), g.id() == old(self).keep.id()
        ensures  self.well_formed()
    {
        let tracked mut tmp = Some(g);
        tracked_swap(&mut self.giveout, &mut tmp); // move token back in
    }

    /// Allocate a brand‑new write token pair (external body trusted)
    #[verifier::external_body]
    proof fn fresh() -> (tracked res: Self)
        ensures res.well_formed()
    {
        Self { keep: KeepToken, giveout: Some(GiveoutToken) }
    }
}

// ─────────────────────────────────────────────────────────────
// Concrete structure S combining actual data Vec with ghost map
// ─────────────────────────────────────────────────────────────

struct S {
    v:   Vec<usize>,
    map: Tracked<Map<nat, WriteToken>>,        // permission map (ghost)
}

impl View for S {
    type V = Map<nat, WriteToken>;
    closed spec fn view(&self) -> Self::V { self.map@ }
}

impl S {
    /// Mutate position `i` after verifying caller holds write permission
    fn set(&mut self, i: usize, t: usize)
        requires i < old(self)@.len(), old(self).can_write(i as _)
    {
        // Implementation left for future: would split token, write vector, merge token
    }

    /// Re‑expose MapWrapper::can_write through S's view
    spec fn can_write(&self, i: nat) -> bool { self@[i].well_formed() }

    /// Basic well‑formedness: map keys exactly match vector indices
    spec fn well_formed(&self) -> bool {
        &&& (forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i))
        &&& (forall|i: nat| i >= self@.len() ==> !self@.dom().contains(i))
    }

    /// Stronger version requiring each entry be well‑formed
    spec fn well_formed_and_complete(&self) -> bool {
        self.well_formed() && (
            forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i) && #[trigger] self@[i].well_formed()
        )
    }
}

// ─────────────────────────────────────────────────────────────
// Small helper: Hole wraps a GiveoutToken and drops it later
// ─────────────────────────────────────────────────────────────

struct Hole { g: Tracked<GiveoutToken> }
impl Hole { fn new(tracked g: GiveoutToken) -> Self { Self { g: Tracked(g) } } }
fn dropHole(h: Hole) -> (res: Tracked<GiveoutToken>) { let Hole { g } = h; g }

// Dummy proof helpers --------------------------------------------------------
proof fn drop(tracked g: GiveoutToken) { }

/// Quick unit proof exercising split / merge rules
proof fn test_tokens() {
    let tracked mut t1 = WriteToken::fresh();
    let tracked mut t2 = WriteToken::fresh();

    let tracked g1 = t1.split();
    let tracked g2 = t2.split();

    drop(g1); // reader drops token, writer still cannot merge without it
    // t1.merge(g1); // would re‑enable writer — commented out intentionally
    // t1.merge(g2); // invalid: id mismatch, caught by verifier
}

}
