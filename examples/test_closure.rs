use vstd::prelude::*;
// container
// no unwind
// trait objects(Verus does not support trait objects now)
verus!{

fn vec_map<T, U>(v: &Vec<T>, f: impl Fn(T) -> U) -> (result: Vec<U>) where
    T: Copy,

    requires
        forall|i|
            0 <= i < v.len() ==> call_requires(
                f,
                (#[trigger] v[i],),
            ),
    ensures
        result.len() == v.len(),
        forall|i|
            0 <= i < v.len() ==> call_ensures(
                f,
                (v[i],),
                #[trigger] result[i],
            )
        ,
{
    let mut result = Vec::new();
    let mut j = 0;
    while j < v.len()
        invariant
            forall|i| 0 <= i < v.len() ==> call_requires(f, (#[trigger] v[i],)),
            0 <= j <= v.len(),
            j == result.len(),
            forall|i| 0 <= i < j ==> call_ensures(f, (v[i],), #[trigger] result[i]),
    {
        result.push(f(v[j]));
        j += 1;
    }
    result
}

trait Ani {
    fn f(&self) {}
}
//
// fn trait_test(a: Box<dyn Ani>) {
//     a.f()
// }

fn trait_tes() {
  // let a: impl Ani;
}
fn trait_tes_iter(v: Vec<usize>) {
   for i in v {
    }
  // let a: impl Ani;
}
// struct Dog;
// impl Ani for Dog {}

// fn trait_test() -> impl Ani {
//         Dog
// }

}

fn main(){}
