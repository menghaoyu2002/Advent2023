use std::{
    fs::File,
    io::{self, Read},
};

fn get_number_of_ways(time: u64, distance: u64) -> u64 {
    let mut sum = 0;

    for i in 0..=time {
        let speed = i;
        let remaining_time = time - i;

        if speed * remaining_time > distance {
            sum += 1;
        }
    }

    sum
}

fn main() -> io::Result<()> {
    let mut file = File::open("./input")?;
    let mut s = String::new();
    _ = file.read_to_string(&mut s);

    let [time, distance, _] = s.split("\n").collect::<Vec<&str>>()[..] else {
        unreachable!();
    };

    let time: Vec<u64> = time
        .strip_prefix("Time:")
        .unwrap()
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect();

    let distance: Vec<u64> = distance
        .strip_prefix("Distance:")
        .unwrap()
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.parse().unwrap())
        .collect();

    assert!(time.len() == distance.len());

    let mut product = 0;
    for i in 0..time.len() {
        product = if product > 0 {
            product * get_number_of_ways(time[i], distance[i])
        } else {
            get_number_of_ways(time[i], distance[i])
        };
    }

    println!("part1 naive: {}", product);

    let time = time.into_iter().reduce(|acc, e| acc * 10u64.pow(e.to_string().len() as u32) + e).unwrap();
    let distance = distance.into_iter().reduce(|acc, e| acc * 10u64.pow(e.to_string().len() as u32) + e).unwrap();

    println!("part2 naive: {}", get_number_of_ways(time, distance));

    // println!("time: {:?}", time);
    // println!("distance: {:?}", distance);
    Ok(())
}
