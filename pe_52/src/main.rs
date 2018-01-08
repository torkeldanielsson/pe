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

fn main() {

    let end = 7;

    for i in 1..999999999999 {

        let digits = split_digits(i);
        let mut is_same_digits: bool = true;

        'mut_loop: for n in 2..end {

            let m_digits = split_digits(n * i);

            if digits.len() != m_digits.len() {
                is_same_digits = false;
                break 'mut_loop;
            }

            for d in &digits {
                let mut found = false;
                for md in &m_digits {
                    if md == d {
                        if found {
                            is_same_digits = false;
                            break 'mut_loop;
                        }
                        found = true;
                    }
                }
                if !found {
                    is_same_digits = false;
                    break 'mut_loop;
                }
            }

            for md in &m_digits {
                let mut found = false;
                for d in &digits {
                    if d == md {
                        if found {
                            is_same_digits = false;
                            break 'mut_loop;
                        }
                        found = true;
                    }
                }
                if !found {
                    is_same_digits = false;
                    break 'mut_loop;
                }
            }
        }

        if is_same_digits {
            for n in 1..end {
                print!("{:?} ", n * i);
            }
            println!("");
            return;
        }
    }
}
