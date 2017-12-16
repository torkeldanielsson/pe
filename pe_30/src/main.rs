fn main() {
    let mut sum: i64 = 0;
    for n in 2..900000 {
        let num_string = n.to_string();
        let mut p: i64 = 0;
        for b in num_string.bytes() {
            let d: i64 = b as i64 - 48;
            p += d.pow(5);
        }
        if p == n {
            sum += n;
            println!("{}: {}", n, p);
        }
    }
    println!("sum: {:?}", sum);
}
