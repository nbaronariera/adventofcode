use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

type Coord = (u8, u8);

#[derive(Clone, Debug)]
enum Tile {
    Wall,
    Box,
    LeftBox,
    RightBox,
}

#[derive(Clone, Debug)]
enum Instructions{
    Right,
    Left,
    Down,
    Up,
}

#[derive(Clone, Debug)]
struct Map{
    special_tiles : HashMap<Coord, Tile>,
    robot: Coord,
    instructions: VecDeque<Instructions>
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

fn part2(input : &Map) -> u32{
    let mut map = fatmap(&mut input.clone());

    0
}

fn fatmap(map : &mut Map) -> Map{

}

fn part1(input : &Map) -> u32{
    let mut map = input.clone();

    while !map.instructions.is_empty(){
        let order = map.instructions.pop_front().unwrap();
        let movement;

        match order {
            Instructions::Right => {movement = (1, 0);},
            Instructions::Left => {movement = (-1, 0);},
            Instructions::Down => {movement = (0, 1);},
            Instructions::Up => {movement = (0, -1);},
        }

        move_robot(&mut map, movement);
    }

    let mut value = 0;
    for ((x, y), tile) in map.special_tiles {
        match tile {
            Tile::Wall => {},
            Tile::Box => {value += 100 * y as u32 + x as u32;},
            _ => {}
        }
    }
    value
}

fn move_robot(map: &mut Map, movement: (i8, i8)){
    let newpos = add_pos(map.robot, movement);

    if map.special_tiles.contains_key(&newpos){
        match map.special_tiles.get(&newpos).unwrap() {
            Tile::Wall => {},
            Tile::Box => {
                if let Some(pos) = can_move_box(&map, newpos, movement) {
                    map.robot = newpos;
                    map.special_tiles.remove(&newpos);
                    map.special_tiles.insert(pos, Tile::Box);
                }
            },
            _ => {}
        }
    }
    else{
        map.robot = newpos;
    }

}

fn can_move_box(map: &Map, pos : Coord, movement : (i8, i8)) -> Option<Coord>{
    if map.special_tiles.contains_key(&pos){
        match map.special_tiles.get(&pos).unwrap() {
            Tile::Wall => {return None;},
            Tile::Box => {return can_move_box(map, add_pos(pos, movement), movement);},
            _ => {}
        }
    }
    
    Some(pos)
}

fn add_pos(x : Coord, y : (i8, i8)) -> Coord{
    let n = (x.0 as i8 + y.0, x.1 as i8 + y.1); 
    (n.0 as u8, n.1 as u8)
}

fn parse(input : &str) -> Map{
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let map_raw = blocks[0];
    let inst_raw = blocks[1];

    let mut robot = (0,0);
    let mut special_tiles: HashMap<Coord, Tile> = HashMap::new();

    for (y, line) in map_raw.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            match c {
                '#' => {special_tiles.insert((x as u8,y as u8), Tile::Wall);}
                'O' => {special_tiles.insert((x as u8,y as u8), Tile::Box);}
                '@' => {robot = (x as u8, y as u8);}
                _ => {}
            }
        }
    }

    let mut instructions:VecDeque<Instructions> = VecDeque::new();
    for line in inst_raw.lines(){
        for c in line.chars(){
            match c {
               '<' => {instructions.push_back(Instructions::Left);} 
               '>' => {instructions.push_back(Instructions::Right);}
               '^' => {instructions.push_back(Instructions::Up);}
               'v' => {instructions.push_back(Instructions::Down);}
               _ => {}
            }
        }
    }

    Map{special_tiles, robot, instructions}

}


#[cfg(test)]
mod test{

    #[test]
    fn test_parse(){
    }

    #[test]
    fn test_part1(){
    }

    #[test]
    fn test_part2(){
    }

}