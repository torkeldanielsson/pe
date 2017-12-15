use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let mut res = 0;

    for i in 1..1000000 {
        let mut len = 1;
        let mut c: i64 = i as i64;
        while c != 1 {
            // print!("{}->", c);
            len += 1;
            if c % 2 == 0 {
                c = c/2;
            } else {
                c = 3*c + 1;
            }

            if len % 1000 == 0 {
                // print!("...{:?}", len);
            }

            if map.get(&c) != None {
                len += map.get(&c).unwrap() - 1;
                c = 1;
            }
        }
        map.insert(i, len);
        // println!("1 ({})", len);
        if len > res {
            res = len;
            println!("{} ({})", i, len);
        }
        if i % 1000 == 0 {
            // println!("({:?})", i);
        }
    }
}
