use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    p1();
    p2();
}

fn p2() {
    let mut position: i32 = 50;
    let mut result = 0;

    let f = File::open("./input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        if line.is_empty() {
            break;
        }

        let direction = if line.chars().nth(0).expect("Expected line") == 'L' {
            -1
        } else {
            1
        };

        let movement = line
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .expect("Expected number")
            * direction;

        let absolute_pos = position + movement;

        let start_lap = if direction == 1 {
            position.div_euclid(100)
        } else {
            (position - 1).div_euclid(100)
        };

        let end_lap = if direction == 1 {
            absolute_pos.div_euclid(100)
        } else {
            (absolute_pos - 1).div_euclid(100)
        };

        result += (end_lap - start_lap).abs();

        position = absolute_pos.rem_euclid(100);
    }
    println!("Resultado p2 {result}");
}

fn p1() {
    let mut position: i32 = 50;
    let mut result = 0;

    let f = File::open("./input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        if line.is_empty() {
            break;
        }

        let direction = if line.chars().nth(0).expect("Expected line") == 'L' {
            -1
        } else {
            1
        };

        let movement = line
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .expect("Expected number")
            * direction;

        position = (position + movement).rem_euclid(100);

        if position == 0 {
            result += 1;
        }
    }

    println!("Resultado p1 {result}");
}
