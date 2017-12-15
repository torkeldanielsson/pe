fn is_palindrome(num: u64) -> bool {
    let num_string = num.to_string();
    let half = num_string.len() / 2;

    num_string.bytes().take(half).eq(num_string.bytes().rev().take(half))
}

fn main() {

    let mut res = 0;

    let mut a = 999;

    while a > 0 {

        let mut b = 999;

        while b > 0 {
            if is_palindrome(a * b) && a * b > res {
                res = a * b;
            }
            b -= 1;
        }

        a -= 1;
    }

    print!("{}\n", res);
}
