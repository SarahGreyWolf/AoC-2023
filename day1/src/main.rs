use std::fs::File;
use std::io::{self, Read};

const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

fn main() -> io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    part1(&input);

    Ok(())
}

fn part1(input: &str) {
    let lines = input.lines();
    let mut final_numbers = vec![];
    for line in lines.into_iter() {
        let numbers = find_numbers(line);
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

fn find_numbers(input: &str) -> Vec<char> {
    let mut numbers: Vec<char> = vec![];
    let mut iter = input.chars().into_iter();
    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            numbers.push(c);
        }
    }
    numbers
}
