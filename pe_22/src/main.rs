use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
use std::collections::HashMap;

fn main() {
    let mut char_to_val = HashMap::new();
    let mut pos = 1;
    for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        char_to_val.insert(ch, pos);
        pos += 1;
    }

    let input = File::open("p022_names.txt").unwrap();
    let reader = BufReader::new(&input);

    let mut names: Vec<String> = Vec::new();

    for name_raw in reader.split(b',') {
        let name_ref = name_raw.unwrap();
        let name: String = str::from_utf8(&name_ref[1..(name_ref.len() - 1)]).unwrap().to_owned();
        names.push(name);
        // println!("{:?}", name);
    }
    names.sort();
    pos = 1;
    let mut res: i64 = 0;
    for name in names {
        let mut c = 0;
        print!("{:?}: ", pos);
        let mut name_sum = 0;
        while c < name.len() {
            let ch = name.chars().nth(c).unwrap();
            print!("{:?}:{} ", ch, char_to_val[&ch]);
            name_sum += char_to_val[&ch];
            c += 1;
        }
        res += name_sum * pos as i64;
        println!("{:?}: {} ({}) (({}))", name, name_sum, name_sum * pos, res);
        pos += 1;
    }
}
