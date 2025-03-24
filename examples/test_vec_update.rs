use vstd::prelude::*;
verus!{
    #[verifier::external_body]
    fn update(v: &mut Vec<usize>, mut index: usize, pos: usize) 
    requires 0 < index < pos < old(v).len()
    ensures v@ =~= old(v)@.update(index as int, old(v)@[pos as int]).update(pos as int, old(v)@[index as int]),
    {
    }

    fn brea(v: &mut Vec<usize>, mut i: usize, j: usize) 
    requires 0 < i < j < old(v).len()
    {
        update(v, i, j);
        assert(v@.subrange(0, i as int) =~= old(v)@.subrange(0, i as int));
    }

    proof fn two(s: Seq<usize>, b: Seq<usize>, i: usize, j: usize)
    requires s.len() == b.len(), s.len() > 0, 0 < i < j < s.len()
    {
        assume(s.subrange(0, j as _) =~= b.subrange(0, j as _));
        assert(s.subrange(0, i as _) =~= b.subrange(0, i as _));
    }
}
