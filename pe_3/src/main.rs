fn main() {
    let target_prime: i64 = 600851475143;

    let mut remainder: i64 = target_prime;

    let mut v: i64 = 1;

    let mut res = 0;

    while v < remainder {
        v = v + 1;

        while remainder % v == 0 {
            res = v;
            remainder = remainder / v;

            print!("{}  ", res);
        }
    }

    print!("\n{}\n", res);
}
