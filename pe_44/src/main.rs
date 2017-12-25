use std::collections::HashSet;

fn main() {
    let mut pentagon_nums_set = HashSet::new();
    let mut pentagon_nums_vec = Vec::new();
    for n in 1..10000 {
        let pentagon_num = n*(3*n-1)/2;
        pentagon_nums_set.insert(pentagon_num);
        pentagon_nums_vec.push(pentagon_num);
        // println!("{:?}", pentagon_num);
    }

    let mut min_diff = 9999999999999;

    for ind_1 in 0..(pentagon_nums_vec.len()/2) {
        for ind_2 in (ind_1 + 1)..(pentagon_nums_vec.len()/2) {
            let p_1 = pentagon_nums_vec[ind_1];
            let p_2 = pentagon_nums_vec[ind_2];
            let sum = p_1 + p_2;
            let diff = p_2 - p_1;
            assert!(diff > 0);
            if pentagon_nums_set.contains(&sum) && pentagon_nums_set.contains(&diff) {
                println!("{} / {}", p_1, p_2);
                if diff < min_diff {
                    min_diff = diff;
                }
            } 
        }
    }
    println!("min_diff: {}", min_diff);
}
