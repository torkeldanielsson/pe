fn paths(m: i64, n: i64, acc: i64) -> i64 {
    if m == 0 || n == 0 {
        return acc;
    }

    return paths(m - 1, n, acc) + paths(m, n - 1, acc);
}

fn main() {
    println!("{}", paths(20, 20, 1));
}
