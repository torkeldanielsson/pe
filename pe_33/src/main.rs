fn main() {
    for n in 11..51 {
        for d in n..101 {

            let mut one_matching = false;
            let mut match_byte = 0;
            for n_b in n.to_string().bytes() {
                for d_b in d.to_string().bytes() {
                    if n_b == d_b && n_b != 48 {
                        one_matching = true;
                        match_byte = n_b;
                    }
                }
            }

            let mut ok = true;
            if n % 11 == 0 ||
               d % 11 == 0 ||
               n >= d {
                ok = false;
            }

            if one_matching && ok {

                let mut other_n = 0;
                let mut other_d = 0;
                for n_b in n.to_string().bytes() {
                    for d_b in d.to_string().bytes() {
                        if n_b != match_byte && d_b != match_byte {
                            other_n = n_b - 48;
                            other_d = d_b - 48;
                        }
                    }
                }

                if other_n != 0 && other_d != 0 {
                    let ratio1: f32 = n as f32 / d as f32;
                    let ratio2: f32 = other_n as f32 / other_d as f32;

                    if (ratio2 - ratio1).abs() < 0.0000001 {
                        println!("{} / {} -> {} / {}", n, d, other_n, other_d);
                    }
                }
            }
        }
    }
}
