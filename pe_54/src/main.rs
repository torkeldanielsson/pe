use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
use std::collections::{HashSet, HashMap};

fn main() {
    let input = File::open("p054_poker.txt").unwrap();
    let reader = BufReader::new(&input);

    let mut hands: Vec<String> = Vec::new();

    for hand_raw in reader.split(b'\n') {
        let hand_ref = word_raw.unwrap();
        let hand: String = str::from_utf8(&hand_ref[1..(hand_ref.len() - 1)]).unwrap().to_owned();
        println!("hand: {:?}", hand);
        hands.push(hand);
    }
}
