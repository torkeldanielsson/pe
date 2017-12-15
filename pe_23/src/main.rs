fn sum_of_proper_divisors(d: i64) -> i64 {
    let mut res: i64 = 0;
    let mut t: i64 = 0;
    while t < d / 2 {
        t += 1;
        if d % t == 0 {
            res += t;
        }
    }

    return res;
}

fn main() {

    let mut abundant_nums: Vec<i64> = Vec::new();

    for i in 1..28123 {
        if i < sum_of_proper_divisors(i) {
            // println!("{}", i);
            abundant_nums.push(i);
        }
    }

    let mut sums: Vec<i64> = Vec::new();

    let mut s: usize = 0; 
    for a in abundant_nums.clone() {
        for b_i in s..abundant_nums.len() {
            sums.push(a + abundant_nums[b_i].clone());
        }
        s += 1;
    }

    sums.sort();

    for s in sums.clone() {
        // println!("{:?}", s);
    }

    let mut res: i64 = 0;

    for i in 0..28123 as i64 {
        if !sums.contains(&i) {
            res += i;
            println!("{:?} ({})", i, res);
        }
    }
}
