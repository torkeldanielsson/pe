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
    let mut count = 3;

    let mut sum: u64 = 2;

    while count < 2000000 {

        if is_prime(count) {
            sum += count;
        }
 
        count += 2;
   }

    println!("{}", sum);
}
