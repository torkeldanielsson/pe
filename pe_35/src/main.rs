fn is_prime(n: i64) -> bool {

    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n <= 1 {
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

    let mut res = 0;
    for n in 1..1000000 {
        if is_prime(n) {

            let mut all_prime = true;

            let len = n.to_string().len();
            for start_i in 0..len {

                let mut s: String = String::new();
                let bytes_b: Vec<char> = n.to_string().chars().collect();

                let mut bytes: Vec<char> = Vec::new();

                for b in bytes_b {
                    bytes.push(b);
                }

                for b_i in start_i..(len + start_i) {
                    let d = bytes[b_i % len];
                    s = format!("{}{}", s, d);
                }

                let shifted_i = s.parse::<i64>().unwrap();
                if !is_prime(shifted_i) {
                    all_prime = false;
                }
            }

            if all_prime {
                res += 1;
                println!("{}", n);
            }
        }
    }

    println!("count: {:?}", res);
}
