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

    let max = 1000000;

    let mut primes = Vec::new();
    for i in 1..(max + 1) {
        if is_prime(i) {
            primes.push(i);
        }
    }

    let mut max = 82;

    for len in 540..550 {
        for start_i in 0..(primes.len() - len) {
            let mut sum = 0;
            for index in start_i..(start_i + len) {
                sum += primes[index];
            }
            if primes.contains(&sum) {
                if len > max {
                    max = len;
                    println!("{:?}: {} terms, start at {}", sum, len, primes[start_i]);
                }
            }
        }
    }
}
