fn main() {
    let mut a = 1;
    while a < 333 {
        let mut b = a + 1;

        while b < (1000 - a - b) {
            let c = 1000 - a - b;

            if (a*a + b*b) == c*c {
                println!("{}, {}, {} ({})", a, b, c, a * b * c);
            }
 
            b += 1;
        }
        a += 1;
    }
}
