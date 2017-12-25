fn is_right_angled(a: i32, b: i32, c: i32) -> bool {
    if a*a + b*b == c*c {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut res = 0;
    let mut max_observed = 0;
    for sum in 3..1000 {
        let mut num = 0;
        for a in 1..(sum/2) {
            for b in 1..(sum/4) {
                let c = sum - a - b;
                if is_right_angled(a, b, c) {
                    num += 1;
                    println!("sum: {}: a:{} b:{} c:{}", sum, a, b, c);
                }
            }
        }
        if num > max_observed {
            res = sum;
            max_observed = num;
            println!("New max: {} at {} combos", res, max_observed);
        }
    }
    println!("Result: {} ({})", res, max_observed);
}
