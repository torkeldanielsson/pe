fn is_prime(n: u64) -> bool {

    if n % 2 == 0 {
        return false;
    }

    let mut i: u64 = 3;

    while i < n/2 {

        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}

fn main() {
    let mut last_prime = 1;
    let mut count = 1;

    let mut done: bool = false;

    let mut p = 1;

    while !done {

        p += 2;

        if is_prime(p) {
            last_prime = p;
            count += 1;

            println!("{}: {}", count, last_prime);

            if count == 10001 {
                done = true;
            }
        }
    }
}
