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

fn is_pandigital(n: i64, digits: i32) -> bool {

    let s = n.to_string();
    let sr: &str = &s;

    if sr.len() != digits as usize {
        return false;
    }
    for i in 1..(digits + 1) {
        let sr_i = i.to_string();
        //println!("{:?}: {}: sr {} cont {}", n, digits, sr, sr_i);
        if !sr.contains(&sr_i) {
            return false;
        }
    }
    return true;
}

fn main() {
    for n in 1..100000000 {
        if is_pandigital(n, n.to_string().len() as i32) && is_prime(n) {
            println!("{}", n);
        }
    }
}
