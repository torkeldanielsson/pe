extern crate bit_vec;
use bit_vec::BitVec;
use std::collections::HashSet;

fn split_digits(n: i64) -> Vec<u8> {
    fn x_inner(n: i64, xs: &mut Vec<u8>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push((n % 10) as u8);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn join_digits(d: Vec<u8>) -> i64 {
    let mut res: i64 = 0;
    for dig in d {
        res *= 10;
        res += dig as i64;
    }
    return res
}

fn main() {

    for max_o in 1..5 {
        let mut max: usize = 10;
        for _i in 0..max_o {
            max *= 10;
        }
        let min: usize = max / 10;

        println!("checking {} - {}", min, max);

        let limit = (max as f64).sqrt() as usize + 1;
        let sievebound: usize = (limit - 1) / 2;  // last index of sieve
        let mut sieve = BitVec::from_elem(sievebound as usize, true);
        let crosslimit: usize = ((limit as f64).sqrt() as usize + 1) / 2;

        for d in 1..crosslimit {
            if sieve[d] { // 2*i + 1 is prime, mark multiples

                let mut i: usize = 2 * d * (d + 1);
                while i < sievebound {
                    sieve.set(i, false);
                    i += 2*d + 1;
                }
            }
        }

        let mut div_primes = Vec::new();
        div_primes.push(2);

        for pi in 1..(sievebound - 1) {
            if sieve[pi] {
                let p = 2*pi + 1;

                div_primes.push(p);
            }
        }

        println!("div_primes ready");

        let mut p = min - 2;
        if p % 2 == 0 {
            p += 1;
        }

        let precomp_primes_range: usize = 100000000;
        let mut precomp_primes_start: usize = 0;
        let mut precomp_primes_end: usize = 0;
        let mut precomp_primes = HashSet::new();

        let mut printed_results = HashSet::new();

        'outer: while p < max {
            p += 2;

            if p > precomp_primes_end || p < precomp_primes_start {
                precomp_primes_start = p;
                precomp_primes_end = p + precomp_primes_range;

                if precomp_primes_end > max {
                    precomp_primes_end = max;
                }

                precomp_primes = HashSet::new();
                
                'precomp: for pi in precomp_primes_start..(precomp_primes_end + 1) {
                    'pcf: for pri in &div_primes {
                        if pri >= &pi {
                            break 'pcf;
                        }
                        if pi % pri == 0 {
                            continue 'precomp;
                        }
                    }
                    precomp_primes.insert(pi);
                }
                println!("precomped {} - {}", precomp_primes_start, precomp_primes_end);
            }

            if !precomp_primes.contains(&p) {
                continue 'outer;
            }

            let digits = split_digits(p as i64);

            if digits.len() > 1 {

                for i1 in 0..(digits.len() - 1) {
                    for i2 in (i1 + 1)..(digits.len() - 0) {
                        if digits[i1] == digits[i2] {
                            let mut count = 0;
                            let mut start_i = 0;
                            if i1 == 0 {
                                start_i = 1;
                            }
                            'inner: for i in start_i..10 {
                                let mut test_d = digits.clone();
                                test_d[i1] = i;
                                test_d[i2] = i;
                                let test_dig = join_digits(test_d) as usize;
                                if test_dig < precomp_primes_start || test_dig > precomp_primes_end {
                                    for pri in &div_primes {
                                        if pri < &test_dig && test_dig % pri == 0 {
                                            continue 'inner;
                                        }
                                    }
                                    count += 1;
                                } else {
                                    if precomp_primes.contains(&test_dig) {
                                        count += 1;
                                    }
                                }
                            }
                            if count >= 7 {
                                if !printed_results.contains(&p) {
                                    print!("{:?}:", count);
                                    'inner2: for i in start_i..10 {
                                        let mut test_d = digits.clone();
                                        test_d[i1] = i;
                                        test_d[i2] = i;
                                        let test_dig = join_digits(test_d) as usize;
                                        for pri in &div_primes {
                                            if pri < &test_dig && test_dig % pri == 0 {
                                                continue 'inner2;
                                            }
                                        }
                                        printed_results.insert(test_dig);
                                        print!(" {:?}", test_dig);
                                    }
                                    println!("");
                                }
                            }
                            if count >= 8 {
                                return
                            }
                        }
                    }
                }
            }
        }
    }
}
