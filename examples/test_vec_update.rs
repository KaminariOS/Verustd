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

    // proof fn two(s: Seq<usize>, b: Seq<usize>, i: usize, j: usize)
    // requires s.len() == b.len(), s.len() > 0, 0 < i < j < s.len()
    // {
    //     assume(s.subrange(0, j as _) =~= b.subrange(0, j as _));
    //     assert(s.subrange(0, i as _) =~= b.subrange(0, i as _));
    // }

    proof fn two1(s: Seq<usize>, b: Seq<usize>, i: usize, j: usize)
    requires s.len() == b.len(), s.len() > 0, 0 < i < j < s.len()
    {
        assume(s.subrange(0, j as _) =~= b.subrange(0, j as _));
        let sp = s.subrange(0, j as _); 
        let bp = b.subrange(0, j as _);
        // assert(s.take( j as _) =~= b.take( j as _));
        assert((forall|k: nat| 0 <= k < j ==>
        (
            #[trigger]
            sp[k as int] == bp[k as int] 
        )
        ));
    }
    
    spec fn parent(i: nat) -> int {
        if i == 0 {
            0
        } else {
            ((i - 1) / 2) as int
        }
    }

    spec fn bi(s: usize, b: usize) -> bool;
    spec fn well_formed_to(s: Seq<usize>, p: int) -> bool {
       if p == 0 {true} else 
        {
             forall|i: nat|  0 <= i < p ==>  #[trigger] bi(s[i as _] , s[parent(i) as _])
         } 
     }

    proof fn well_formed_subrange(s: Seq<usize>, prefix: int, len: int) 
            requires well_formed_to(s, len as _), 0 <= prefix <= len
        ensures well_formed_to(s, prefix as _)
    {

    }

    struct S {
        v: Vec<usize>
    }

    fn mutate_inner(s: &mut S) {
    }

    fn mutate(s: &mut S) 
    // ensures something
    {
        // let ghost old_s = old(s);
        loop {
            // let ghost curent = s; // This errors 
            mutate_inner(s);
            // Cannot call s(before mutate_inner) spec and proof 
            // Want to compare s(after mutate_inner) and s(before mutate_inner) here to prove to
            // something
        }
        
    }
}
