fn main() {

    let mut res: i64 = 0;

    for i in 0..1000000 {
        let b = format!("{:b}", i);
        if i.to_string() == i.to_string().chars().rev().collect::<String>() &&
            b == b.chars().rev().collect::<String>() {

            println!("{:?} / {:?}", i, b);
            res += i;
        }
    }

    println!("{:?}", res);
}
