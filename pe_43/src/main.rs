fn is_pandigital(s: &str) -> bool {

    if s.len() == 10 &&
        s.contains('0') &&
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
    let mut res: i64 = 0;
    for n in 1023456789..9876543210 {
        //let n = 1406357289;

        let mut is_ok = true;
        let s = n.to_string();
        if is_pandigital(&s) {

            let mut digits: Vec<i32> = Vec::new();
            for b in s.bytes() {
                digits.push(b as i32 - 48);
            }

            if (digits[1]*100 + digits[2]*10 + digits[3]) % 2 != 0 {
                is_ok = false;
            }

            if (digits[2]*100 + digits[3]*10 + digits[4]) % 3 != 0 {
                is_ok = false;
            }

            if (digits[3]*100 + digits[4]*10 + digits[5]) % 5 != 0 {
                is_ok = false;
            }

            if (digits[4]*100 + digits[5]*10 + digits[6]) % 7 != 0 {
                is_ok = false;
            }

            if (digits[5]*100 + digits[6]*10 + digits[7]) % 11 != 0 {
                is_ok = false;
            }

            if (digits[6]*100 + digits[7]*10 + digits[8]) % 13 != 0 {
                is_ok = false;
            }

            if (digits[7]*100 + digits[8]*10 + digits[9]) % 17 != 0 {
                is_ok = false;
            }

        } else {
            is_ok = false;
        }

        if is_ok {
            println!("{}", n);
            res += n;
        }
    }
    println!("res: {:?}", res);
}
