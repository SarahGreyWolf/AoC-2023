#![feature(iter_advance_by)]

use std::fs::File;
use std::io::{self, Read};
use std::iter::Peekable;
use std::str::Chars;

const EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    evaluate(&input, false);
    evaluate(&input, true);

    Ok(())
}

fn evaluate(input: &str, part2: bool) {
    let lines = input.lines();
    let mut final_numbers = vec![];
    for line in lines.into_iter() {
        let numbers = find_numbers(line, part2);
        let mut total = String::new();
        if let Some(first) = numbers.first() {
            total.push(*first);
        }
        if let Some(last) = numbers.last() {
            total.push(*last);
        }
        final_numbers.push(u64::from_str_radix(&total, 10).unwrap());
    }
    let result = final_numbers.iter().fold(0, |acc, x| acc + x);
    println!("{result}");
}

fn find_numbers(input: &str, part2: bool) -> Vec<char> {
    // PART 1
    let mut numbers: Vec<char> = vec![];
    let mut peekable = input.chars().into_iter().peekable();
    while let Some(c) = peekable.peek() {
        let c = *c;
        if c.is_digit(10) {
            numbers.push(c);
            peekable.next().unwrap();
        } else {
            if part2 {
                // PART 2
                let value = find_number_words(&mut peekable);
                if let Some(value) = value {
                    numbers.push(value);
                } else {
                    peekable.next().unwrap();
                }
            } else {
                peekable.next().unwrap();
            }
        }
    }
    numbers
}

// PART 2
fn find_number_words(peekable: &mut Peekable<Chars<'_>>) -> Option<char> {
    'words: for (index, word) in WORDS.iter().enumerate() {
        let mut peekable_clone = peekable.clone();
        let chars = word.as_bytes();
        for c in chars {
            let Some(peeked) = peekable_clone.peek() else {
                continue 'words;
            };
            if *peeked as u8 == *c {
                peekable_clone.next().unwrap();
            } else {
                continue 'words;
            }
        }
        peekable.advance_by(word.len() - 1).unwrap();
        return char::from_digit((index as u64 + 1) as u32, 10);
    }

    None
}
