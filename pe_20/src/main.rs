extern crate num;
use num::bigint::BigInt;

fn main() {
    let mut big: BigInt = 1.into();

    for i in 1..101 {
        big = big * i;
    }
    
    let digits: Vec<_> = big.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    
    let mut res: u32 = 0;
    
    for d in digits {
        res += d;
    }

    println!("{:?}", res);
}
