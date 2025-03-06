use vstd::prelude::*;
verus!{
    // fn overflow() 
    // -> u8
    // no_unwind
    // {
    //     let i: u8 = 255;
    //     let mut j = 0;
    //     for k in 0..i 
    //         invariant 
    //             k <= u8::max,
    //             j <= u8::max,
    //     {
    //         j =  k;
    //     }
    //     j
    // }

    // fn overflow_1(k: u8) 
    // -> u8
    // requires k as nat + 5 <= u8::MAX
    // no_unwind
    // {
    //     k + 5
    // }
    
    // fn overflow_1<T>(k: Option<T>) 
    // -> T
    // requires k.is_some()
    // no_unwind
    // {
    //     k.unwrap()
    // }
}

fn main() {
    // println!("{}", overflow());
}
