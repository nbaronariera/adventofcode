use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Machine{
    button_a: (i8, i8),
    button_b: (i8, i8),
    prize: (i64, i64),
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    println!("Datos parseados en {} ms", t.elapsed().as_millis());

    let t = Instant::now();
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part1(input : &Vec<Machine>) -> i32{
    let mut tokens = 0;

    for machine in input{
        let det = (machine.button_a.0 as i32 * machine.button_b.1 as i32) - (machine.button_a.1 as i32 * machine.button_b.0 as i32);
        
        if det == 0 {panic!("Det == 0");}

        let x = (machine.prize.0 as i32 * machine.button_b.1 as i32) - (machine.button_b.0  as i32 * machine.prize.1 as i32);
        let y = (machine.prize.1 as i32 * machine.button_a.0 as i32) - (machine.button_a.1  as i32 * machine.prize.0 as i32);

        if x % det != 0 || y % det != 0 {continue;}

        let x = x / det;
        let y = y / det;

        if x <= 100 && y <= 100 && x > 0 && y > 0 {tokens += (x * 3) + y;}
    }

    tokens
}

fn part2(input : &Vec<Machine>) -> i64{
    let mut val = input.clone();

    for machine in val.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    }

    let mut tokens = 0;

    for machine in val{
        let det = (machine.button_a.0 as i64 * machine.button_b.1 as i64) - (machine.button_a.1 as i64 * machine.button_b.0 as i64);
        
        if det == 0 {panic!("Det == 0");}

        let x = (machine.prize.0 as i64 * machine.button_b.1 as i64) - (machine.button_b.0  as i64 * machine.prize.1 as i64);
        let y = (machine.prize.1 as i64 * machine.button_a.0 as i64) - (machine.button_a.1  as i64 * machine.prize.0 as i64);

        if x % det != 0 || y % det != 0 {continue;}

        let x = x / det;
        let y = y / det;

        if x > 0 && y > 0 {tokens += (x * 3) + y;}
    }

    tokens
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .collect::<Vec<_>>()  
        .chunks(4)
        .map(|chunk| Machine {
            button_a: parse_button(chunk[0]),
            button_b: parse_button(chunk[1]),
            prize:    parse_prize(chunk[2]),
        })
        .collect()
}

fn parse_button(line: &str) -> (i8, i8) {
    let parts: Vec<&str> = line.split(':').nth(1).unwrap().split(',').collect();
    (
        parts[0].split('+').nth(1).unwrap().parse::<i8>().unwrap(),
        parts[1].split('+').nth(1).unwrap().parse::<i8>().unwrap(),
    )
}

fn parse_prize(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(':').nth(1).unwrap().split(',').collect();
    (
        parts[0].split('=').nth(1).unwrap().parse::<i64>().unwrap(),
        parts[1].split('=').nth(1).unwrap().parse::<i64>().unwrap(),
    )
}


#[cfg(test)]
mod test{
    use std::fs;

    use crate::{parse, part1, Machine};


    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTest.txt").unwrap();

        let machine = Machine{
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        
        assert_eq!(parse(&text), vec![machine]);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest2.txt").unwrap();

        assert_eq!(part1(&parse(&text)), 480);
    }

    #[test]
    fn test_part2(){
    }

}