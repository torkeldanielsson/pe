fn main() {
    let mut max_so_far = 0;
    for i in 1..1000 {
        let mut digits = Vec::new();
        let mut mem = Vec::new();
        let mut n: i64 = 1;
        let d: i64 = i;
        let mut rep = 0;
        for _c in 0..2000000 {
            let t: i64 = n / d;
            digits.push(t);
            n = n % d;
            n *= 10;
            let mut mem_rev = mem.clone();
            mem_rev.reverse();
            for mem_p in 0..mem_rev.len() {
                if mem_rev[mem_p] == n {
                    rep = mem_p + 1;
                }
            }
            if rep != 0 {
                break;
            }
            if n == 0 {
                digits.push(t);
                break;
            }
            mem.push(n);
        }

        if rep > max_so_far {
            max_so_far = rep;

            print!("1 / {} = ", i);
            let mut max_len = digits.len();
            let mut trunkate = false;
            let max_d_len = 20;
            if max_len > max_d_len {
                max_len = max_d_len - 3;
                trunkate = true;
            }
            for c in 0..max_len {
                print!("{}", digits[c]);
                if c == 0 && digits.len() > 1 {
                    print!(".");
                }
                if c + 1 == digits.len() - rep && rep != 0 {
                    print!("(");
                }
            }
            if trunkate {
                print!("...");
            }
            if rep != 0 {
                print!(")");
            }
            let mut print_space = max_d_len - max_len;
            if rep == 0 {
                print_space += 2;
            }
            while print_space > 0 {
                print_space -= 1;
                print!(" ");
            }
            if rep != 0 {
                print!("\t{}", rep);
            }
            println!("");
            // println!("{:?}", digits);
            // println!("{:?}", mem);
        }
    }
}
