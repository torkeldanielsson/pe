fn is_pandigital(s: &str) -> bool {

    if s.len() == 9 &&
        s.contains('1') &&
        s.contains('2') &&
        s.contains('3') &&
        s.contains('4') &&
        s.contains('5') &&
        s.contains('6') &&
        s.contains('7') &&
        s.contains('8') &&
        s.contains('9') {

        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut res = 0;
    for i in 0..99999 {
        let mut s = String::new();
        let mut d = 1;
        while s.len() < 9 {
            s = format!("{}{}", s, (d*i).to_string());
            d += 1;
        }
        if is_pandigital(&s) {
            println!("i: {} d: {} s: {}", i, d, s);
            let t = s.parse::<i32>().unwrap();
            if t > res {
                res = t;
            }
        }
    }
    println!("{:?}", res);
}
