use vstd::prelude::*;
use vstd::modes::tracked_swap;
verus!{

tracked struct KeepToken;

impl KeepToken {
    spec fn id(&self) -> int;
}

tracked struct GiveoutToken;

impl GiveoutToken {
    spec fn id(&self) -> int;
}

tracked struct MapWrapper {
    inner_map: Map<nat, WriteToken>
}

impl View for MapWrapper {
    type V = Map<nat, WriteToken>;
    closed spec fn view(&self) -> Self::V {
        self.inner_map
    }
}

impl MapWrapper {
    proof fn empty() -> Self {
        Self {
            inner_map: Map::tracked_empty()
        }
    }
//     proof fn allocate(tracked &mut self)
//             requires old(self).well_formed()
//             ensures old(self)@.len() + 1 == self@.len(), 
// // self.well_formed() 
//         {
//         let len = self@.len();
//         assert(!self@.dom().contains(len));
//         self.inner_map.tracked_insert(len, WriteToken::fresh());
//         assert(len == self@.len() + 1);
//     }

    fn deallocate(&mut self) {
    }
    // spec fn can_read(i : nat) -> bool {
    //     self@[i]
    // }
    spec fn can_write(&self, i : nat) -> bool {
        self@[i].well_formed()
    }
    

    spec fn well_formed(&self) -> bool {
        &&& (forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i))
        &&& (forall|i: nat|  i >= self@.len() ==> !self@.dom().contains(i))
    }

    spec fn well_formed_and_complete(&self) -> bool {
        &&& (forall|i: nat| 0 <= i < self@.len() ==> self@.dom().contains(i) && #[trigger] self@[i].well_formed())
    }
}

// It seems that a Seq wrapper is better

tracked struct WriteToken {
    keep: KeepToken,
    giveout: Option<GiveoutToken>,
}

impl WriteToken {
    #[verifier::type_invariant]
    spec fn type_inv(&self) -> bool {
        if let Some(g) = self.giveout {
            g.id() == self.keep.id()
        } else {
            true
        }
    }
    spec fn well_formed(&self) -> bool {
        self.giveout.is_some()
    }
    proof fn split(tracked &mut self) -> (tracked res: GiveoutToken) 
    requires old(self).well_formed()
    ensures !self.well_formed(), res == old(self).giveout.get_Some_0()
    , res.id() == self.keep.id(), res.id() == old(self).keep.id()
    {
        use_type_invariant(&*self);
        let tracked mut n = None;
        tracked_swap(&mut self.giveout, &mut n);
        n.tracked_unwrap()
    }

    proof fn merge(tracked &mut self, tracked g: GiveoutToken ) 
    requires !old(self).well_formed(), g.id() == old(self).keep.id()
    ensures self.well_formed(), 
    {
        let tracked mut n = Some(g);
        tracked_swap(&mut self.giveout, &mut n);
    }

    
    #[verifier::external_body]
    proof fn fresh() -> (tracked res: Self)
        ensures res.well_formed()
        {
        Self {
            keep: KeepToken,
            giveout: Some(GiveoutToken)
        }
    }
}

    proof fn drop(tracked g: GiveoutToken) {}
    proof fn test_tokens() {
        let tracked mut t1 = WriteToken::fresh();
        let tracked mut t2 = WriteToken::fresh();
        let tracked g1 = t1.split();
        let tracked g2 = t2.split();
        drop(g1);
        t1.merge(g1);
        // t1.merge(g2);
    }
    

}
