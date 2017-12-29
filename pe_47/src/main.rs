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

fn distinct_factors(n: i64, primes: &Vec<i64>) -> i32 {
    let mut remainder = n;
    let mut distinct_factors = Vec::new();
    while remainder != 1 {
        let limit: i64 = (remainder as f64).sqrt() as i64 + 1;
        for f in primes {
            if remainder % f == 0 || f > &limit {
                if f > &limit {
                    assert!(primes.contains(&remainder));
                    if !distinct_factors.contains(&remainder) {
                        // print!("[{:?}] ", remainder);
                        distinct_factors.push(remainder);
                    }
                    remainder = 1;
                } else {
                    remainder = remainder / f;
                    if !distinct_factors.contains(&f) {
                        // print!("{:?} ", f);
                        distinct_factors.push(f.clone());
                    }
                }
                break;
            }
        }
    }
    return distinct_factors.len() as i32;
}

fn main() {

    let max = 9999999;

    let mut primes = Vec::new();
    let mut odd_composites = Vec::new();
    for i in 1..max {
        if is_prime(i) {
            primes.push(i);
        } else if i%2 != 0 {
            odd_composites.push(i);
        }
    }

    println!("primes calculated");

    let mut sequence_len = 0;
    let mut last = 0;
    for i in 3..max {
        if !primes.contains(&i) {
            let df = distinct_factors(i, &primes);
            if df != 4 {
                sequence_len = 0;
                continue;
            }

            if sequence_len > 0 && last == i - 1 {
                sequence_len += 1;
            } else {
                sequence_len = 1;
            }

            last = i;
                
            if sequence_len == 4 {
                println!("{}, {}, {}, {}", i - 3, i - 2, i - 1, i);
                return;
            }

        } else {
            sequence_len = 0;
        }
    }
}
