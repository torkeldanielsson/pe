use std::char;

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

    let min: i64 =  10000000;
    let max: i64 = 999999999;

    let mut primes = Vec::new();
    let mut c: i64 = min - 1;
    while c < (max + 1) {
        c += 2;
        if is_prime(c) {
            primes.push(c);
        }
    }
    primes.sort();

    println!("generated primes");

    let mut tested = Vec::new();

    for p in primes.clone() {
        let mut chars: Vec<char> = p.to_string().chars().collect();
        for i1 in 0..(chars.len() - 2) {
            for i2 in (i1 + 1)..(chars.len() - 1) {
                if chars[i1] == chars[i2] {
                    let mut test_c = chars.clone();
                    test_c[i1] = '*';
                    test_c[i2] = '*';
                    if !tested.contains(&test_c) {
                        tested.push(test_c.clone());

                        let mut count = 0;
                        let mut digs = Vec::new();
                        let mut start_n = 0;
                        if i1 == 0 {
                            start_n = 1;
                        }
                        for n in start_n..10 {
                            chars[i1] = char::from_digit(n, 10).unwrap();
                            chars[i2] = char::from_digit(n, 10).unwrap();
                            let s: String = chars.iter().collect();
                            let to_test: i64 = s.parse::<i64>().unwrap();
                            if primes.contains(&to_test) {
                                count += 1;
                                digs.push(n);
                            }
                        }

                        if count >= 8 {
                            let s: String = test_c.iter().collect();
                            print!("{}: {} -", count, s);
                            for d in digs {
                                print!(" {}", d);
                            }
                            println!("");
                        }
                    }
                }
            }
        }
    }
}
