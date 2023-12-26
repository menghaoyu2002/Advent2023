use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fs::File,
    io::{self, Read},
};

use queues::{IsQueue, Queue};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_string(&self) -> &str {
        match self {
            Up => "u",
            Down => "d",
            Left => "l",
            Right => "r",
        }
    }
}

fn get_next_direction(dir: &Direction, pipe: &str) -> Option<Direction> {
    match dir {
        Up => match pipe {
            "|" => Some(Up),
            "J" => Some(Right),
            "L" => Some(Left),
            _ => None,
        },
        Left => match pipe {
            "-" => Some(Left),
            "J" => Some(Down),
            "7" => Some(Up),
            _ => None,
        },
        Right => match pipe {
            "-" => Some(Right),
            "F" => Some(Up),
            "L" => Some(Down),
            _ => None,
        },
        Down => match pipe {
            "|" => Some(Down),
            "7" => Some(Right),
            "F" => Some(Left),
            _ => None,
        },
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Up => Up,
            Down => Down,
            Left => Left,
            Right => Right,
        }
    }
}

use Direction::*;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y > other.y {
            return Some(Ordering::Greater);
        }

        if self.y < other.y {
            return Some(Ordering::Less);
        }

        Some(self.x.cmp(&other.x))
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn get_starting_pos(pipes: &Vec<Vec<&str>>) -> Option<Point> {
    for i in 0..pipes.len() {
        for j in 0..pipes[0].len() {
            if pipes[i][j] == "S" {
                return Some(Point {
                    y: i as u32,
                    x: j as u32,
                });
            }
        }
    }

    None
}

fn is_vertical(dir: &Direction) -> bool {
    match dir {
        Up => true,
        Down => true,
        _ => false,
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("./input")?;
    let mut s = String::new();

    let _ = file.read_to_string(&mut s);

    let pipes: Vec<Vec<&str>> = s
        .split("\n")
        .map(|row| row.split("").filter(|c| *c != "").collect::<Vec<&str>>())
        .filter(|a| !a.is_empty())
        .collect();

    let starting_pos = get_starting_pos(&pipes).unwrap();

    let mut queue: Queue<(Point, Direction, Vec<(Point, Direction)>)> = Queue::new();

    _ = queue.add((
        Point {
            x: starting_pos.x,
            y: starting_pos.y + 1,
        },
        Up,
        vec![(Point { ..starting_pos }, Up)],
    ));
    _ = queue.add((
        Point {
            x: starting_pos.x + 1,
            y: starting_pos.y,
        },
        Left,
        vec![(Point { ..starting_pos }, Left)],
    ));

    if starting_pos.y > 0 {
        _ = queue.add((
            Point {
                x: starting_pos.x,
                y: starting_pos.y - 1,
            },
            Down,
            vec![(Point { ..starting_pos }, Down)],
        ));
    }
    _ = queue.add((
        Point {
            x: starting_pos.x - 1,
            y: starting_pos.y,
        },
        Right,
        vec![(Point { ..starting_pos }, Right)],
    ));

    let path = loop {
        let (pos, curr_dir, mut path) = queue.remove().unwrap();
        let pipe = pipes[pos.y as usize][pos.x as usize];

        let next_dir = get_next_direction(&curr_dir, pipe);

        if pipe == "S" {
            break path;
        }

        match next_dir {
            Some(dir) => {
                let mut point = pos.clone();
                match dir {
                    Up => point.y += 1,
                    Down => point.y -= 1,
                    Left => point.x += 1,
                    Right => point.x -= 1,
                };

                if is_vertical(&dir) {
                    path.push((pos.clone(), dir.clone()));
                } else {
                    path.push((pos.clone(), curr_dir.clone()));
                }

                let _ = queue.add((point, dir, path));
            }
            None => {}
        }
    };

    println!("part 1: {:?}", path.len() / 2);

    let mut result_area = pipes.clone();

    // look for corners
    let mut refined_path = vec![];
    let mut i = 0;
    while i < path.len() {
        let (point, dir) = &path[i];
        let next = path.get(i + 1);

        let pipe = pipes[point.y as usize][point.x as usize];
        if pipe == "|" || pipe == "S" {
            refined_path.push(path[i].clone());
        } else if is_vertical(dir) && next.is_some() {
            let (mut next_point, mut next_dir) = path[i + 1].clone();

            while !is_vertical(&next_dir) && i < path.len() {
                i += 1;
                match path.get(i + 1) {
                    Some((point, dir)) => {
                        next_dir = dir.clone();
                        next_point = point.clone();
                    }
                    _ => {
                        break;
                    }
                }
            }
            i += 1;

            if is_vertical(&next_dir) && point.y == next_point.y {
                if next_dir == *dir {
                    match dir {
                        Up => refined_path.push((
                            Point {
                                x: next_point.x.min(point.x),
                                y: point.y,
                            },
                            dir.clone(),
                        )),
                        Down => refined_path.push((
                            Point {
                                x: next_point.x.max(point.x),
                                y: point.y,
                            },
                            dir.clone(),
                        )),
                        _ => unreachable!(),
                    }
                }
            } else {
                refined_path.push((point.clone(), dir.clone()));
            }
        } else {
            refined_path.push(path[i].clone());
        }
        i += 1;
    }

    let points: BTreeSet<Point> = path
        .iter()
        .map(|(p, _)| p.clone())
        .collect::<BTreeSet<Point>>();

    const X: &str = "\x1b[91mX\x1b[0m";

    let mut rows = vec![vec![]; pipes.len()];
    let mut sum = 0;
    for i in 0..refined_path.len() {
        let (p2, dir) = &refined_path[i];
        let y = p2.y;

        // result_area[p2.y as usize][p2.x as usize] = dir.to_string();

        if is_vertical(dir) {
            rows[y as usize].push(refined_path[i].clone());
        }
    }

    for row in rows {
        let mut s = row.clone();
        s.sort_by(|a, b| a.0.cmp(&b.0));
        let mut stack = vec![];

        for (p2, dir) in s {
            let mut changed = false;
            match stack.last() {
                Some((_, last)) => {
                    if *last != dir {
                        let (p1, _): (Point, Direction) = stack.pop().unwrap();
                        // println!("{:?} {:?}", p1, p2);

                        let y = p2.y;

                        let start = p1.x.min(p2.x);
                        let end = p1.x.max(p2.x);

                        for i in start..end {
                            if !points.contains(&Point { y, x: i }) {
                                result_area[y as usize][i as usize] = X;
                            }
                        }
                        changed = true;
                    }
                }
                _ => {}
            }

            if !changed {
                stack.push((p2, dir));
            }
        }
    }

    println!();
    for line in result_area {
        println!("{}", line.join(""));
        for char in line {
            if char == X {
                sum += 1;
            }
        }
    }
    println!();

    println!("part 2: {:?}", sum);

    Ok(())
}
