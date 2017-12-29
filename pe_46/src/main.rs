fn is_prime(n: i64) -> bool {

    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n <= 1 {
        return false;
    }

    let mut i: i64 = 3;

    while i <= (n as f64).sqrt() as i64 {

        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}

fn main() {
    let mut primes = Vec::new();
    let mut odd_composites = Vec::new();
    for i in 3..9999999 {
        if is_prime(i) {
            primes.push(i);
        } else if i%2 != 0 {
            odd_composites.push(i);
        }
    }
    for c in odd_composites {
        for p in primes.clone() {
            if p > c {
                println!("first one: {:?}", c);
                return;
            }
            let i: i64 = (((c - p)/2) as f64).sqrt() as i64;
            if (p + 2*i*i) == c {
                println!("{} = {} + 2*{}^2", c, p, i);
                break;
            }
        }
    }
}
