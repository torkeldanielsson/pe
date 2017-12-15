use std::collections::HashMap;

fn main() {
    let ones: [String; 10] = ["".to_owned(), "one".to_owned(), "two".to_owned(), "three".to_owned(), "four".to_owned(), "five".to_owned(), "six".to_owned(), "seven".to_owned(), "eight".to_owned(), "nine".to_owned()];
    let mut map = HashMap::new();
    for i in 1..10 {
        map.insert(i, ones[i].clone());
    }
    map.insert(10, "ten".to_owned());
    map.insert(11, "eleven".to_owned());
    map.insert(12, "twelve".to_owned());
    map.insert(13, "thirteen".to_owned());
    map.insert(14, "fourteen".to_owned());
    map.insert(15, "fifteen".to_owned());
    map.insert(16, "sixteen".to_owned());
    map.insert(17, "seventeen".to_owned());
    map.insert(18, "eighteen".to_owned());
    map.insert(19, "nineteen".to_owned());
    for i in 0..10 {
        map.insert(20 + i, format!("{}{}", "twenty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(30 + i, format!("{}{}", "thirty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(40 + i, format!("{}{}", "forty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(50 + i, format!("{}{}", "fifty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(60 + i, format!("{}{}", "sixty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(70 + i, format!("{}{}", "seventy".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(80 + i, format!("{}{}", "eighty".to_owned(), ones[i].clone()));
    }
    for i in 0..10 {
        map.insert(90 + i, format!("{}{}", "ninety".to_owned(), ones[i].clone()));
    }
    for h in 1..10 {
        map.insert(h*100, format!("{}{}", ones[h].clone(), "hundred".to_owned()));
        for i in 1..100 {
            let num: String = map.get(&i).unwrap().to_owned();
            map.insert(h*100 + i, format!("{}{}{}", ones[h].clone(), "hundredand".to_owned(), num));
        }
    }
    map.insert(1000, "onethousand".to_owned());
    let mut sum = 0;
    for i in 1..1001 {
        println!("{:?} ({})", map.get(&i).unwrap(), map.get(&i).unwrap().len());
        sum += map.get(&i).unwrap().len();
    }
    println!("{:?}", sum);
}
