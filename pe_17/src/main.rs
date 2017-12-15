use std::collections::HashMap;

fn main() {
    let ones: [&str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut map = HashMap::new();
    for i in 1..10 {
        map.insert(i, ones[i].to_owned());
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
        map.insert(20 + i, format!("{}{}", "twenty", ones[i]));
    }
    for i in 0..10 {
        map.insert(30 + i, format!("{}{}", "thirty", ones[i]));
    }
    for i in 0..10 {
        map.insert(40 + i, format!("{}{}", "forty", ones[i]));
    }
    for i in 0..10 {
        map.insert(50 + i, format!("{}{}", "fifty", ones[i]));
    }
    for i in 0..10 {
        map.insert(60 + i, format!("{}{}", "sixty", ones[i]));
    }
    for i in 0..10 {
        map.insert(70 + i, format!("{}{}", "seventy", ones[i]));
    }
    for i in 0..10 {
        map.insert(80 + i, format!("{}{}", "eighty", ones[i]));
    }
    for i in 0..10 {
        map.insert(90 + i, format!("{}{}", "ninety", ones[i]));
    }
    for h in 1..10 {
        map.insert(h*100, format!("{}{}", ones[h], "hundred"));
        for i in 1..100 {
            let num = format!("{}{}{}", ones[h], "hundredand", &map.get(&i).unwrap());
            map.insert(h*100 + i, num);
        }
    }
    map.insert(1000, "onethousand".to_owned());
    let mut sum = 0;
    for i in 1..1001 {
        //println!("{:?} ({})", map.get(&i).unwrap(), map.get(&i).unwrap().len());
        sum += map.get(&i).unwrap().len();
    }
    println!("{:?}", sum);
}
