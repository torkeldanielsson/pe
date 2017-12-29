extern crate num;
use num::bigint::BigInt;

fn main() {
    let mut res: BigInt = 0.into();
    for i in 1..1001 {
        let big_add: BigInt = num::pow(i.into(), i);
        // println!("+ {}^{} = {}", i, i, big_add);
        res = res + big_add;
    }
    println!("= {}", res.to_string());
}
