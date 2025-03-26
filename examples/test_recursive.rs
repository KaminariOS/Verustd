use vstd::prelude::*;
verus!{
spec fn reverse(xs: Seq<int>) -> Seq<int>
    decreases xs,
{
    if xs.len() == 0 {
        xs
    } else {
        reverse(xs.drop_first()).push(xs.first())
    }
}

proof fn reverse_id(x: int) ensures reverse(seq![x]) =~= seq![x] {

        reveal_with_fuel(reverse, 2);
}

}
