use vstd::prelude::*;
verus!{

    #[verifier::external_body]
    fn mut_test(x: usize) {
        let mut v = vec!["fd".to_owned(), "kd".to_owned(), "jd".to_owned()];
        let mut s = "sdf".to_owned();
        core::mem::swap(&mut s, &mut v[1]);
    }
}

fn main(){}
