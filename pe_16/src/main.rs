extern crate num;
use num::bigint::BigInt;

fn main() {
    let two: BigInt = 2.into();
    let mut big: BigInt = 2.into();

    for _i in 1..1000 {
        big = big * &two;
    }
    
    let digits: Vec<_> = big.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    
    let mut res: u32 = 0;
    
    for d in digits {
        res += d;
    }

    println!("{:?}", res);
}
