extern crate num;
use num::bigint::BigInt;

fn main() {
    let mut i: i64 = 3;
    let mut f_m2: BigInt = "1".parse::<BigInt>().unwrap();
    let mut f_m1: BigInt = "1".parse::<BigInt>().unwrap();
    let mut f_m0: BigInt = "2".parse::<BigInt>().unwrap();

    println!("F1: {:?}", f_m2.to_string());
    println!("F2: {:?}", f_m1.to_string());
    println!("F3: {:?}", f_m0.to_string());

    while f_m0.to_string().len() < 1000 {
        f_m2 = f_m1;
        f_m1 = f_m0;
        f_m0 = f_m1.clone() + f_m2;
        i += 1;
        println!("F{}: {:?}", i, f_m0.to_string());
    }
}
