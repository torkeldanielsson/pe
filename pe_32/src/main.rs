fn main() {
    let a_max = 987;
    let b_max = 9876;

    let mut products = Vec::new();

    for a in 1..a_max {

        let mut used_digits = Vec::new();
        used_digits.push(0);

        let mut a_ok = true;
        for byte in a.to_string().bytes() {
            let d = byte - 48;
            if used_digits.contains(&d) {
                a_ok = false;
            }
            used_digits.push(d);
        }

        if a_ok {
            let a_len = a.to_string().len();

            for b in 1..b_max {

                let mut used_digits_for_b = used_digits.to_vec();

                let mut b_ok = true;

                for byte in b.to_string().bytes() {
                    let d = byte - 48;
                    if used_digits_for_b.contains(&d) {
                        b_ok = false;
                    }
                    used_digits_for_b.push(d);
                }

                if a > b {
                    b_ok = false;
                }

                if b_ok {

                    let b_len = b.to_string().len();
                    let mut used_digits_for_c = used_digits_for_b.to_vec();

                    let c = a * b;

                    let c_len = c.to_string().len();

                    if a_len + b_len + c_len == 9 {

                        let mut c_ok = true;

                        for byte in c.to_string().bytes() {
                            let d = byte - 48;
                            if used_digits_for_c.contains(&d) {
                                c_ok = false;
                            }
                            used_digits_for_c.push(d);
                        }

                        if c_ok {
                            if !products.contains(&c) {
                                products.push(c)
                            }
                            println!("{} * {} = {}", a, b, c);
                        }
                    }
                }
            }
        }
    }

    let mut res = 0;
    for p in products {
        res += p;
    }
    println!("product sum {:?}", res);
}
