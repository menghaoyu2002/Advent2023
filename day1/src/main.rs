use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./src/input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for input in reader.lines() {
        let line = input.unwrap();
        let (first, last) = get_first_and_last_digit(&line).unwrap();
        sum += first * 10 + last;
        println!("{} {} {}", first, last, line);
    }
    println!("{}", sum);

    Ok(())
}

fn is_digit_word(s: &str, start: usize) -> Option<u32> {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut i = 0;
    while i < DIGITS.len() {
        let digit = DIGITS[i];
        if start + digit.len() <= s.len() && s[start..start + digit.len()] == *digit {
            return Some((i + 1).try_into().unwrap());
        }

        i += 1;
    }

    None
}

fn get_first_and_last_digit(s: &str) -> Option<(u32, u32)> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    let mut pos = 0;
    for char in s.chars() {
        let digit = if char.is_digit(10) {
            char.to_digit(10)
        } else {
            is_digit_word(&s, pos)
        };

        if digit.is_some() {
            last = digit;
            if first.is_none() {
                first = digit;
            }
        }

        pos += 1;
    }

    if first.is_some() && last.is_some() {
        return Some((first?, last?));
    }

    None
}
