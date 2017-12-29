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

    let max = 9999;

    let mut primes = Vec::new();
    for i in 1..max {
        if is_prime(i) {
            primes.push(i);
        }
    }

    for a in &primes {
        if a.to_string().len() > 3 {
            for b in &primes {
                if b > a {
                    let c = b + (b - a);
                    if primes.contains(&c) {
                        let mut sa: Vec<_> = a.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                        sa.sort();
                        let mut sb: Vec<_> = b.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                        sb.sort();
                        let mut sc: Vec<_> = c.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
                        sc.sort();
                        if sa == sb && sa == sc {
                            println!("{}{}{} ({})", a, b, c, b - a);
                        }
                    }
                }
            }
        }
    }
}
