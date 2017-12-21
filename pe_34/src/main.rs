fn factorial(n: i64) -> i64 {
    if n <= 0 {
        return 1;
    }

    let mut res = 1;
    for i in 1..(n+1) {
        res *= i;
    }
    return res;
}

fn recursive_checker(acc: i64, fact_sum_acc: i64, depth: i64) {
    if depth > 0 {
        for d in 0..10 {
            let new_acc = 10*acc + d;
            let new_fact_sum_acc = fact_sum_acc + factorial(d);
            if new_acc == new_fact_sum_acc {
                println!("{}", new_acc);
            }
            recursive_checker(new_acc, new_fact_sum_acc, depth - 1)
        }
    }
}

fn main() {
    
    recursive_checker(0, 0, 10);
}
