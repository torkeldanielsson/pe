extern crate bit_vec;
use bit_vec::BitVec;

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

    let min: i64 =  9999999999;
    let max: i64 = 99999999999;

    let primes = {

        let mut bv = BitVec::from_elem(max as usize, true);

        bv.set(0, false);
        bv.set(1, false);

        for i in 2..(1 + (max as f64).sqrt() as usize) {
            if bv[i] {
                for j in i.. {
                    if i * j >= (max as usize) {
                        break;
                    }
                    bv.set(i * j, false)
                }
            }
        }
        bv
    };

    println!("primes calculated");

    // for p in 0..max {
    //     if primes[p as usize] {
    //         println!("{:?} {}", p, join_digits(split_digits(p)));
    //     }
    // }

    for p in 0..max {

        if primes[p as usize] && p > min {
            let digits = split_digits(p);

            if digits.len() > 2 {

                for i1 in 0..(digits.len() - 2) {
                    for i2 in (i1 + 1)..(digits.len() - 1) {
                        if digits[i1] == digits[i2] {
                            let mut count = 0;
                            let mut start_i = 0;
                            if i1 == 0 {
                                start_i = 1;
                            }
                            for i in start_i..10 {
                                let mut test_d = digits.clone();
                                test_d[i1] = i;
                                test_d[i2] = i;
                                let test_dig = join_digits(test_d);
                                if test_dig < max && primes[test_dig as usize] {
                                    count += 1;
                                }
                            }
                            if count >= 7 {
                                let mut test_d1 = digits.clone();
                                test_d1[i1] = 0;
                                test_d1[i2] = 0;
                                let mut test_d2 = digits.clone();
                                test_d2[i1] = 1;
                                test_d2[i2] = 1;
                                println!("{}: {} / {} / {}", count, join_digits(digits.clone()), join_digits(test_d1), join_digits(test_d2));
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
