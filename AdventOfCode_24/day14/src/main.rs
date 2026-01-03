use std::collections::{HashMap, HashSet};
use std::{fs, u8};
use std::time::Instant;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Robot{
    pos: (u8, u8),
    velocity: (i8, i8)
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Map{
    width: u8,
    height: u8,
    robots: Vec<Robot>
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text, 103, 101);
    println!("Datos parseados en {} ms", t.elapsed().as_millis());

    let t = Instant::now();
    let r1 = part1(&input, 100);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part1(input : &Map, time: u16) -> u32{
    let mut dict:HashMap<(u8, u8), u16> = HashMap::new();

    for robot in input.robots.iter(){
        let x = (robot.pos.0 as i32 + (robot.velocity.0 as i32 * time as i32)).rem_euclid(input.width as i32);
        let y = (robot.pos.1 as i32 + (robot.velocity.1 as i32 * time as i32)).rem_euclid(input.height as i32);

        dict.entry((x as u8, y as u8)).and_modify(|x| *x += 1).or_insert(1);
    }

    read_quadrant((0,0), (input.width/2-1, input.height/2-1), &dict) *
    read_quadrant((input.width/2+1, 0), (input.width, input.height/2-1), &dict) *
    read_quadrant((0, input.height/2+1), (input.width/2-1, input.height), &dict) *
    read_quadrant((input.width/2+1, input.height/2+1), (input.width, input.height), &dict)
}

fn part2(input : &Map) -> u32{
    let mut entries = input.robots.clone();
    let mut count = 0;

    loop {
        let mut positions = HashSet::new();
        for robot in entries.iter_mut(){
            
            let x = (robot.pos.0 as i32 + robot.velocity.0 as i32).rem_euclid(input.width as i32);
            let y = (robot.pos.1 as i32 + robot.velocity.1 as i32).rem_euclid(input.height as i32);

            robot.pos.0 = x as u8;
            robot.pos.1 = y as u8;
            positions.insert((robot.pos.0, robot.pos.1));
        }

        count += 1;
        let mut found = false;

        for (x,y) in &positions{
            let mut no_tree = false;
            for i in 0..=7{
                if !positions.contains(&(*ax, y+i)) {no_tree = true;}
            }

            if !no_tree{found = true; break;}
        }        
        if found {break;}
    }
    count
}

fn read_quadrant(from: (u8, u8), to: (u8, u8), dict: &HashMap<(u8, u8), u16>) -> u32 {
    let mut count: u32 = 0;
    for i in from.0..=to.0{
        for j in from.1..=to.1{
            count += *dict.get(&(i, j)).unwrap_or(&0) as u32;
        }
    } 
    count
} 

fn parse(input : &str, height: u8, width: u8) -> Map{ 
    let robots = input.lines().map({
        |x| 
        Robot {
            pos: get_pos(x),
            velocity: get_velocity(x)
        }
    }).collect::<Vec<Robot>>();

    Map {height, width, robots}
}

fn get_pos(input: &str) -> (u8, u8){
    let mut values = input.split_whitespace().nth(0).unwrap().split("=").nth(1).unwrap().split(",");
    (values.next().unwrap().parse::<u8>().unwrap(), values.next().unwrap().parse::<u8>().unwrap())
}

fn get_velocity(input: &str) -> (i8, i8){
    let mut values = input.split_whitespace().nth(1).unwrap().split("=").nth(1).unwrap().split(",");
    (values.next().unwrap().parse::<i8>().unwrap(), values.next().unwrap().parse::<i8>().unwrap())
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, Robot};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTestParse.txt").unwrap();
        let map = parse(&text, 7, 11);
        assert_eq!(map.robots, vec![
            Robot{pos:(0, 4), velocity:(3,-3)},
            Robot{pos:(6,3), velocity:(-1,-3)},
            Robot{pos:(10, 3), velocity:(-1,2)},
        ]);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part1(&parse(&text, 7, 11), 100), 12);
    }

    #[test]
    fn test_part2(){
    }

}