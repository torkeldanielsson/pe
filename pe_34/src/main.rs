use std::collections::HashMap;

fn factorial(n: i64) -> i64 {
    if n <= 0 {
        return 1;
    }

    let mut res = 1;
    for i in 1..(n+1) {
        res *= i;
    }
    return res;
}

fn main() {
    let mut f_map = HashMap::new();
    for i in 0..10 {
        f_map.insert(i, factorial(i));
    }

    let mut res = 0;
    
    for d in 3..9999999 {
        let mut factorial_sum: i64 = 0;
        for digit_b in d.to_string().bytes() {
            let digit: i64 = digit_b as i64 - 48;
            factorial_sum += f_map.get(&digit).unwrap();
        }

        if d == factorial_sum {
            print!("{}", d);
            let mut first = true;
            for digit_b in d.to_string().bytes() {
                let digit = digit_b - 48;
                if first {
                    first = false;
                    print!(" = ");
                } else {
                    print!(" + ");
                }
                print!("{}!", digit);
            }
            println!("");

            res += d;
        }
    }

    println!("{:?}", res);
}
