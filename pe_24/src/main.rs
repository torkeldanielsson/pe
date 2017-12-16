fn permutate(choices: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    if choices.len() == 1 {
        res.push(choices);
        return res;
    }

    for choice in choices.clone() {
        let mut choices_left: Vec<i32> = Vec::new();
        for i in choices.clone() {
            if i != choice {
                choices_left.push(i);
            }
        }
        let more_perms: Vec<Vec<i32>> = permutate(choices_left);
        for perm_end in more_perms {
            let mut perm: Vec<i32> = Vec::new();
            perm.push(choice);
            for p in perm_end {
                perm.push(p);
            }
            res.push(perm);
        }
    }

    return res;
}

fn main() {

    let mut input: Vec<i32> = Vec::new();
    for i in 0..10 {
        input.push(i);
    }

    let mut res: Vec<Vec<i32>> = permutate(input);
    res.sort();
    for i in (res[999999]).clone() {
        print!("{:?}", i);
    }
    println!("{:?}", res[999999]);
}
