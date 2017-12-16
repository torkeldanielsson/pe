fn main() {
    let mut res: i64 = 1;
    let mut skip: i64 = 2;
    let mut p: i64 = 1;
    println!("1");
    for _s in 1..501 {
        for _i in 0..4 {
            p += skip;
            res += p;
            println!("{:?}", p);
        }
        skip += 2;
    }
    println!("{:?}", res);
}
