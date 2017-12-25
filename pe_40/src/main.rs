fn champer_n(n: i64) -> i64 {
    let mut c = 1;
    let mut c_len: i64 = 1;
    let mut offset: i64 = 0;
    while offset + c_len < n {
        c += 1;
        offset += c_len;
        c_len = c.to_string().len() as i64;
    }

    let bytes_b: Vec<char> = c.to_string().chars().collect();
    let mut bytes: Vec<char> = Vec::new();
    for b in bytes_b {
        bytes.push(b);
    }
    return bytes[(n - offset) as usize - 1] as i64 - 48;
}

fn main() {
/*
    for i in 1..20 {
        println!("{}:\t{}", i, champer_n(i));
    }
*/

    let mut res = 1;
    for p in 0..7 {
        let mut pw: i64 = 1;
        for _i in 0..p {
            pw *= 10;
        }
        let champ = champer_n(pw);
        println!("{} D {}: {}", p, pw, champ);
        res *= champ;
    }
    println!("res: {:?}", res);
}
