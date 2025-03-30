use vstd::prelude::*;
verus!{
    fn func_with_a_loop() 
    -> usize
    no_unwind
    {
        let i: usize = 255;
        let mut j = 0;
        let mut k1 = 0;
        while k1 < i
            invariant 
        
        decreases i - k1
        {
            k1 += 1;
            j =  k1;
        }
        j
    }

    fn div(k: u8, j: u8) 
    -> u8
    requires k as nat + 5 <= u8::MAX, j != 0
    no_unwind
    {
        k / j
    }

    fn overflow_1(k: u8) 
    -> u8
    requires k as nat + 5 <= u8::MAX
    no_unwind
    {
        k + 5
    }
    
    // fn unwrap<T>(k: Option<T>) 
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
