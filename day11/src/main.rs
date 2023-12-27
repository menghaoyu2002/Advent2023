use std::{
    fs::File,
    io::{self, Read},
};

type Point = (usize, usize);

fn get_all_galaxies(input: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxies = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    galaxies
}

fn get_min_distance(
    start: &Point,
    end: &Point,
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    part: usize,
) -> usize {
    let distance = end.0.abs_diff(start.0) + end.1.abs_diff(start.1);

    let mut row_intersections = 0;
    for x in start.1.min(end.1)..start.1.max(end.1) {
        if empty_cols.binary_search(&x).is_ok() {
            row_intersections += 1;
        }
    }

    let mut col_intersections = 0;
    for y in start.0.min(end.0)..start.0.max(end.0) {
        if empty_rows.binary_search(&y).is_ok() {
            col_intersections += 1;
        }
    }

    distance - row_intersections - col_intersections
        + row_intersections * part
        + col_intersections * part
}

fn main() -> io::Result<()> {
    let mut file = File::open("./input")?;
    let mut s = String::new();

    let _ = file.read_to_string(&mut s);

    let mut empty_rows = vec![];
    let mut input = vec![];
    let mut i = 0;
    for line in s.split("\n").filter(|s| *s != "") {
        let mut row = vec![];
        let mut is_empty = true;
        for char in line.chars() {
            row.push(char);
            is_empty = is_empty && char == '.';
        }

        if is_empty {
            empty_rows.push(i);
        }

        input.push(row);
        i += 1;
    }

    // expand columns
    let mut empty_cols = vec![];
    for x in 0..input[0].len() {
        let mut is_empty = true;

        for y in 0..input.len() {
            is_empty = is_empty && input[y][x] == '.';
        }

        if is_empty {
            empty_cols.push(x);
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;
    let galaxies = get_all_galaxies(&input);
    for g1 in 0..galaxies.len() {
        for g2 in g1 + 1..galaxies.len() {
            part1 += get_min_distance(&galaxies[g1], &galaxies[g2], &empty_rows, &empty_cols, 2);
            part2 += get_min_distance(
                &galaxies[g1],
                &galaxies[g2],
                &empty_rows,
                &empty_cols,
                1000000,
            );
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
