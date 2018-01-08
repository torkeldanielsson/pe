extern crate num;
use num::bigint::BigInt;

fn factorial(n: i32) -> BigInt {
    if n <= 0 {
        return 1.into();
    }

    let mut res: BigInt = 1.into();
    for i in 1..(n + 1) {
        let ib: BigInt = i.into();
        res = res * ib;
    }
    return res;
}

fn ncr(n: i32, r: i32) -> BigInt {
    assert!(r <= n);
    return factorial(n) / (factorial(r) * factorial(n - r));
}

fn main() {
    let mut res: i32 = 0;
    for n in 1..101 {
        println!("{} ({})", n, res);
        for r in 1..(n + 1) {
            let c = ncr(n, r);
            if c > 1000000.into() {
                res += 1;
            }
        }
    }
    println!("{:?}", res);
}
