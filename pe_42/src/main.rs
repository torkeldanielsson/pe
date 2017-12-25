use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
use std::collections::{HashSet, HashMap};

fn main() {
    let mut char_to_val = HashMap::new();
    let mut pos = 1;
    for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        char_to_val.insert(ch, pos);
        pos += 1;
    }

    let input = File::open("p042_words.txt").unwrap();
    let reader = BufReader::new(&input);

    let mut words: Vec<String> = Vec::new();

    for word_raw in reader.split(b',') {
        let word_ref = word_raw.unwrap();
        let wordt: String = str::from_utf8(&word_ref[1..(word_ref.len() - 1)]).unwrap().to_owned();
        //println!("{:?}", wordt);
        words.push(wordt);
    }

    let mut triangle_nums = HashSet::new();
    for n in 1..1000 {
        triangle_nums.insert(n*(n+1)/2);
    }

    let mut res = 0;

    for w in words {
        let mut wsum = 0;
        for c in w.chars() {
            wsum += char_to_val[&c];
        }
        if triangle_nums.contains(&wsum) {
            res += 1;
            println!("{:?} ({})", w, wsum);
        }
    }
    println!("triangle words: {:?}", res);
}
