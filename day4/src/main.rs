use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut num_of_winnings = vec![];
    let mut sum = 0;
    for input in reader.lines() {
        let line = input?;

        let nums = &line[line.find(": ").unwrap() + 2..];

        let [winnings, actual] = nums.split(" | ").collect::<Vec<&str>>()[..] else {
            unreachable!()
        };

        let mut winning_numbers = HashSet::new();
        for win in winnings.split(" ") {
            match win.parse::<u32>() {
                Ok(num) => {
                    winning_numbers.insert(num);
                }
                _ => {}
            }
        }

        // let mut points = 0;
        let mut winnings = 0;
        for num in actual.split(" ") {
            match num.parse::<u32>() {
                Ok(num) => {
                    if winning_numbers.contains(&num) {
                        // points = if points > 0 { points * 2 } else { 1 };
                        winnings += 1;
                    }
                }
                _ => {}
            }
        }
        // sum += points;
        num_of_winnings.push(winnings);
    }

    for i in 0..num_of_winnings.len() {
        sum += 1;
        let mut stack = vec![i];
        while !stack.is_empty() {
            let card = stack.pop().unwrap();
            sum += num_of_winnings[card];
            for j in card + 1..card + num_of_winnings[card] + 1 {
                stack.push(j);
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
