use vstd::prelude::*;
verus!{
    fn set(v: &mut Vec<usize>) {
        if v.len() > 0 {
            v[0] = 1;
            assert(v[0] == 1);
        }
    }

    #[verifier::external_body]
    fn swap<T>(v: &mut Vec<T>, i: usize, j: usize)
    requires i < old(v).len(), j < old(v).len()
    ensures v@ =~= v@.update(i as int, old(v)@[j as int]).update(j as int, old(v)@[i as int]),
    v.len() == old(v).len()
    {
        v.swap(i, j);
    }


    // pub assume_specification<T>[ <[T]>::swap](v: &mut [T], i: usize, j: usize)
    // requires i < old(v).len(), j < old(v).len()
    // ensures v@ =~= v@.update(i as int, old(v)@[j as int]).update(j as int, old(v)@[i as int]);

    fn swap_test<T>(v: &mut Vec<T>, i: usize, j: usize)
    requires i < old(v).len(), j < old(v).len()
    {
        swap(v, i, j);
        assert(v@[i as int] == old(v)@[j as int]);
    }

    #[verifier::external_body]
    fn test_ghost(v: Ghost<Seq<usize>>) 
    ensures v@.len() == 0
    {

    }

    fn pass_ghost(v: Vec<usize>) {
        test_ghost(Ghost(v@));
        assert(v.len() == 0);
    }

    #[verifier::external_body]
    fn mut_test(x: usize) {
        let mut v = vec!["fd".to_owned(), "kd".to_owned(), "jd".to_owned()];
        let mut s = "sdf".to_owned();
        core::mem::swap(&mut s, &mut v[1]);
    }
}

fn main(){}
