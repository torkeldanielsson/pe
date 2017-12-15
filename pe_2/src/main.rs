fn main() {
    let mut v1 = 1;
    let mut v2 = 2;

    let mut res = 0;

    while v2 < 4000000 {
        if v2 % 2 == 0 {
            res += v2;
        }

        let tmp = v1 + v2;
        v1 = v2;
        v2 = tmp;
    }

    print!("{}\n", res);
}
