use std::io;

const EMPTY: char = '.';
const BRICK: char = '#';
const SNOW: char = '*';

fn main() {
    let mut dimensions = "".to_string();

    io::stdin().read_line(&mut dimensions).unwrap();

    let dimensions = dimensions.split(' ').map(|val| val.trim().parse::<u32>().unwrap()).collect::<Vec<_>>();

    let height = dimensions[0];
    let width = dimensions[1];

    let mut result = Vec::new();

    for _ in 0..height {
        let mut row = Vec::new();

        for _ in 0..width {
            row.push(EMPTY);
        }

        result.push(row);
    };

    let mut snow_count = Vec::new();

    for _ in 0..width {
        snow_count.push(0);
    }

    for i in 0..height {
        let mut row = "".to_string();

        io::stdin().read_line(&mut row).unwrap();

        for (j, block) in row.chars().enumerate() {
            if block == SNOW {
                snow_count[j] += 1;
            }

            if block == BRICK {
                result[i as usize][j] = BRICK;

                for k in i - snow_count[j]..i {
                    result[k as usize][j] = SNOW;
                }

                snow_count[j] = 0;
            }
        }
    }

    for row in result {
        for block in row {
            print!("{}", block);
        }

        println!("");
    }
}
