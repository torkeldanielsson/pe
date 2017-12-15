fn sum_of_proper_divisors(d: i64) -> i64 {
    let mut res: i64 = 0;
    let mut t: i64 = 0;
    while t < d / 2 {
        t += 1;
        if d % t == 0 {
            res += t;
        }
    }

    return res;
}

fn main() {

    let mut res = 0;

    for i in 1..10000 {
        let a = sum_of_proper_divisors(i);
        let b = sum_of_proper_divisors(a);
        if a == b {
            println!("{} / {}", b, a);
            res += i;
        }
    }

    println!("{:?}", res);
}
