fn num_divisors(n: i64) -> i64 {
    let mut res: i64 = 1;
    let mut t: i64 = 0;
    while t < n / 2 {
        t += 1;
        if n % t == 0 {
            res += 1;
        }
    }

    return res;
}

fn main() {
    let mut c: i64 = 0;
    let mut t: i64 = 0;
    let mut n = 0;
    let mut l = 0;

    while n < 500 {
        c += 1;
        t += c;
        n = num_divisors(t);
        if n > l {
            l = n;
            println!("{} ({}) [{}]", n, t, c);
        }
    }
    println!("{:?}", t);
}
