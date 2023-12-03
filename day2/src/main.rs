use std::{
    cmp,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for input in reader.lines() {
        let line = input?;
        let split: Vec<&str> = line.split(": ").collect();
        // let id = split[0].split(" ").collect::<Vec<&str>>()[1];
        let games: Vec<&str> = split[1].split("; ").collect();

        // let is_possible = is_possible(games);
        // if is_possible {
        //     sum += id.parse::<u32>().unwrap();
        // }
        // println!("{}: {}", id, is_possible);

        sum += power_of_minimum_set(games);
    }
    println!("{}", sum);

    Ok(())
}

fn power_of_minimum_set(games: Vec<&str>) -> u32 {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;

    for game in games {
        let results: Vec<&str> = game.split(", ").collect();
        for result in results {
            let pair: Vec<&str> = result.split(" ").collect();

            let num = pair[0].parse::<u32>().unwrap();
            let color = pair[1];

            match color {
                "red" => max_r = cmp::max(max_r, num),
                "green" => max_g = cmp::max(max_g, num),
                "blue" => max_b = cmp::max(max_b, num),
                _ => unreachable!("unknown color {}", color),
            };
        }
    }

    max_r * max_g * max_b
}

fn is_possible(games: Vec<&str>) -> bool {
    for game in games {
        let results: Vec<&str> = game.split(", ").collect();
        for result in results {
            let pair: Vec<&str> = result.split(" ").collect();

            let num = pair[0].parse::<u32>().unwrap();
            let color = pair[1];

            let within_limits = match color {
                "red" => num <= 12,
                "green" => num <= 13,
                "blue" => num <= 14,
                _ => unreachable!("unknown color {}", color),
            };

            if !within_limits {
                return false;
            }
        }
    }

    true
}
