use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut iter = reader.lines();

    let sequence: Vec<char> = iter.next().unwrap()?.chars().collect();

    iter.next();

    let mut starting_points = vec![];
    let mut map = HashMap::new();
    let re = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();
    for input in iter {
        let line = input.unwrap();
        let Some(caps) = re.captures(&line) else {
            unreachable!()
        };

        let key = String::from(&caps[1]);
        let left = String::from(&caps[2]);
        let right = String::from(&caps[3]);

        if key.ends_with("A") {
            starting_points.push(key.clone());
        }

        map.insert(key, (left, right));
    }

    let offset = 0;
    let mut steps = 0;

    let mut curr = "AAA";
    while !curr.ends_with("Z") {
        let Some((left, right)) = map.get(curr) else {
            unreachable!()
        };

        match sequence[(offset + steps) % sequence.len()] {
            'L' => curr = left,
            'R' => curr = right,
            _ => unreachable!(),
        }

        steps += 1;
    }
    println!("part1: {}", steps);

    // naive
    // let mut steps = 0;
    // while !starting_points
    //     .iter()
    //     .fold(true, |acc, s| acc && s.ends_with("Z"))
    // {
    //     for i in 0..starting_points.len() {
    //         let Some((left, right)) = map.get(&starting_points[i]) else {
    //             unreachable!()
    //         };
    //
    //         match sequence[steps % sequence.len()] {
    //             'L' => starting_points[i] = left.to_string(),
    //             'R' => starting_points[i] = right.to_string(),
    //             _ => unreachable!(),
    //         }
    //
    //         steps += 1;
    //     }
    //     println!("{:?}", starting_points[0]);
    // }
    // println!("part2 naive: {}", steps);

    // ??? nowhere did it say we can make this assumption
    // i noticed it in their example but i didn't think we could assume it
    // i gave up and this is what we're doing now
    let mut all_steps = vec![];
    for key in starting_points {
        let mut curr = &key;
        let mut steps = 0;
        while !curr.ends_with("Z") {
            let Some((left, right)) = map.get(curr) else {
                unreachable!()
            };

            match sequence[steps % sequence.len()] {
                'L' => curr = left,
                'R' => curr = right,
                _ => unreachable!(),
            }

            steps += 1;
        }

        all_steps.push(steps);
    }

    let mut lcm = all_steps[0];
    for step in &all_steps {
        lcm = step * lcm / gcd::binary_usize(*step, lcm);
    }

    println!("{}", lcm);
    println!("part2: {:?}", all_steps);

    Ok(())
}
