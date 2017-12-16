extern crate num;
use num::bigint::{BigInt, ToBigInt};

fn main() {
    let mut values = Vec::new();
    for a in 2..101 {
        for b in 2..101 {
            let val: BigInt = num::pow(a.to_bigint().unwrap(), b as usize);
            if !values.contains(&val) {
                values.push(val);
            }
        }
    }
    values.sort();
    println!("{:?}", values);
    println!("terms: {:?}", values.len());
}
