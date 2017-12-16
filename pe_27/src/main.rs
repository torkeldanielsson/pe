fn is_prime(n: i64) -> bool {

    if n % 2 == 0 || n < 0 {
        return false;
    }

    let mut i: i64 = 3;

    while i <= n/2 {

        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}

fn main() {
    let range = 999;
    let mut max_p = 0;
    let mut prod;
    for a in -range..(range + 1) {
        for b in -(range + 1)..(range + 2) {
            let mut i = 0;
            while is_prime(i*i + a*i + b) {
                i += 1;
            }
            if i > max_p {
                max_p = i;
                prod = a*b;
                println!("n^2 + {}n + {}: {} (prod {})", a, b, i, prod);
            }
        }
    }
}
