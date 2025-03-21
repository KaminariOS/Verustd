use vstd::prelude::*;
use vstd::simple_pptr::*;
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

    // fn test_tracked_usize(Tracked(v): Tracked<&mut nat>) requires v == 0
    // {
    //
    // }

    fn test_tracked_points(Tracked(perm): Tracked<&mut PointsTo<u64>>) 
    requires old(perm).is_init() 
    {

    }

    #[verifier::external_body]
    fn increment(counter: PPtr<u64>, Tracked(perm): Tracked<&mut PointsTo<u64>>)
        requires
            counter == old(perm).pptr(),
            old(perm).is_init() && old(perm).value() < 100,
        ensures
            perm.pptr() == old(perm).pptr(),
            perm.opt_value() == MemContents::Init((old(perm).value() + 1) as u64),
    {}

    #[verifier::external_body]
    fn mut_test(x: usize) {
        let mut v = vec!["fd".to_owned(), "kd".to_owned(), "jd".to_owned()];
        let mut s = "sdf".to_owned();
        core::mem::swap(&mut s, &mut v[1]);
    }

    struct Hole {ptr: *mut usize}
    impl Hole {
        #[verifier::external_body]
        fn new(v: &mut Vec<usize>) -> (s: Self) 

        ensures  v@ =~= old(v)@
        {
            Self {
                ptr: v.as_mut_ptr()
            }
        }
        #[verifier::external_body]
        fn get(&self, i: usize) -> &usize {
            unsafe { &*self.ptr.add(i) }
        }

        #[verifier::external_body]
        fn set(&mut self, i: usize, v: usize)  {
            unsafe { *self.ptr.add(i) = v; }
        }
    }

    fn vec_ptr(v: &mut Vec<usize>) {
        let mut h = Hole::new(v);
        let num = h.get(0);
        h.set(0, 1);
        assert(old(v)@ =~= v@);
    }
}

fn main(){}
