use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    vec,
};

fn main() -> io::Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for input in reader.lines() {
        let mut line = vec![];
        for c in input?.chars() {
            line.push(c);
        }
        v.push(line);
    }

    // println!("{:?}", v[0]);

    let mut gears = vec![vec![(0, 1); v[0].len()]; v.len()];
    let mut sum = 0;

    for row in 0..v.len() {
        let mut col = 0;
        while col < v[0].len() {
            let mut num = 0;
            let mut gear_x = None;
            let mut gear_y = None;
            while col < v[0].len() && v[row][col].is_digit(10) {
                num = num * 10 + v[row][col].to_digit(10).unwrap();

                let x_start = if col > 1 { col - 1 } else { 0 };
                let y_start = if row > 1 { row - 1 } else { 0 };
                for x in x_start..col + 2 {
                    for y in y_start..row + 2 {
                        if v.get(y).is_some() {
                            let symbol = v.get(y).unwrap().get(x);

                            if symbol.is_some() && *symbol.unwrap() == '*' {
                                gear_x = Some(x);
                                gear_y = Some(y);
                            }
                        }
                    }
                }

                col += 1;
            }

            if gear_x.is_some() && gear_y.is_some() {
                let (count, val) = gears[gear_y.unwrap()][gear_x.unwrap()];
                gears[gear_y.unwrap()][gear_x.unwrap()] = (count + 1, val * num);
            }

            col += 1;
        }
    }

    for i in 0..v.len() {
        for j in 0..v[0].len() {
            let (count, val) = gears[i][j];
            if count == 2 {
                sum += val;
            }
        }
    }

    // println!("{:?}", gears[1]);
    println!("{}", sum);

    Ok(())
}
