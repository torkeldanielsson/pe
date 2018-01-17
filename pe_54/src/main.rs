use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct Card {
    val: i32,
    suit: i32,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Game {
    h_a: Hand,
    h_b: Hand,
}

fn main() {
    let input = File::open("p054_poker.txt").unwrap();
    let reader = BufReader::new(&input);

    let mut games: Vec<Game> = Vec::new();

    for hand_raw in reader.split(b'\n') {
        let hand_ref = hand_raw.unwrap();
        let hand: String = str::from_utf8(&hand_ref[0..(hand_ref.len())]).unwrap().to_owned();
        let mut count = 0;
        let mut game: Game = Game{ h_a: Hand { cards: Vec::new() }, h_b: Hand { cards: Vec::new() } };
        for card in hand.split(" ") {

            let mut card_obj: Card = Card { val: 0, suit: 0 };

            let val = card.chars().nth(0).unwrap();
            let suit = card.chars().nth(1).unwrap();

            if val == '2' {
                card_obj.val = 2;
            } else if val == '3' {
                card_obj.val = 3;
            } else if val == '4' {
                card_obj.val = 4;
            } else if val == '5' {
                card_obj.val = 5;
            } else if val == '6' {
                card_obj.val = 6;
            } else if val == '7' {
                card_obj.val = 7;
            } else if val == '8' {
                card_obj.val = 8;
            } else if val == '9' {
                card_obj.val = 9;
            } else if val == 'T' {
                card_obj.val = 10;
            } else if val == 'J' {
                card_obj.val = 11;
            } else if val == 'Q' {
                card_obj.val = 12;
            } else if val == 'K' {
                card_obj.val = 13;
            } else if val == 'A' {
                card_obj.val = 14;
            } else {
                println!("VAL FAIL: {}", val);
                return;
            }

            if suit == 'D' {
                card_obj.suit = 1;
            } else if suit == 'C' {
                card_obj.suit = 2;
            } else if suit == 'S' {
                card_obj.suit = 3;
            } else if suit == 'H' {
                card_obj.suit = 4;
            } else {
                println!("SUIT FAIL: {}", suit);
                return;
            }

            print!("v:{} s:{}, ", val, suit);

            if count < 5 {
                game.h_a.cards.push(card_obj);
            } else {
                game.h_b.cards.push(card_obj);
            }

            count += 1;
        }
        println!("\n{:?}", game);
    }
}
