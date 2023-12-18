use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    for input in reader.lines() {
        let line = input?;

        let mut layers = vec![];

        let history: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        layers.push(history);

        let mut prev = &layers[layers.len() - 1];
        while !prev.iter().fold(true, |acc, n| acc && *n == 0) {
            let mut diffs = vec![];

            for i in 1..prev.len() {
                diffs.push(prev[i] - prev[i - 1]);
            }

            layers.push(diffs);
            prev = &layers[layers.len() - 1];
        }

        let mut part1_prev = 0;
        let mut part2_prev = 0;
        for i in (0..layers.len() - 1).rev() {
            part1_prev = layers[i].last().unwrap() + part1_prev;
            part2_prev = layers[i].first().unwrap() - part2_prev;
        }

        part1 += part1_prev;
        part2 += part2_prev;
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    Ok(())
}
