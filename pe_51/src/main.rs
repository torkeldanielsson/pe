use std::collections::HashMap;

fn main() {

    let min: i64 =  9999999999;
    let max: i64 = 49999999999;
    let mut primes: Vec<i64> = Vec::new();

    {
        let mut prime_v: Vec<bool> = Vec::new();
        for _i in 0..max {
            prime_v.push(true);
        }

        println!("vector inited");


        prime_v[1] = false;

        for p in 2..max {
            if prime_v[p as usize] {
                if p > min {
                    primes.push(p);
                }
                let mut c: i64 = 2 * p;
                while c < max {
                    prime_v[c as usize] = false;
                    c += p;
                }
            }
        }

        println!("primes calculated");
    }

    let mut saved_mutations: HashMap<String, i32> = HashMap::new();

    let mut second_digit: char = '0';

    for p in primes {
        let chars: Vec<char> = p.to_string().chars().collect();

        if chars.len() > 2 {

            if chars[1] != second_digit {
                second_digit = chars[1];
                saved_mutations.clear();
            }

            for i1 in 1..(chars.len() - 2) {
                for i2 in (i1 + 1)..(chars.len() - 1) {
                    if chars[i1] == chars[i2] {
                        let mut test_c = chars.clone();
                        test_c[i1] = '*';
                        test_c[i2] = '*';
                        let test_s: String = test_c.iter().collect();
                        if !saved_mutations.contains_key(&test_s) {
                            saved_mutations.insert(test_s, 1);
                        } else {
                            let val: i32 = saved_mutations.get(&test_s).unwrap() + 1;
                            saved_mutations.insert(test_s.clone(), val);
                            if saved_mutations[&test_s] >= 7 {
                                println!("{}: {}", &saved_mutations[&test_s], test_s);
                            }
                            if saved_mutations[&test_s] == 8 {
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
}
