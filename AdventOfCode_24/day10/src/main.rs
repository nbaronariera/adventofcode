use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

type Map = Vec<Vec<Tile>>;
type Tile = u8;
type Coord = (u8, u8);

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&input);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2((map, starts) : &(Map, HashSet<Coord>)) -> u32{
    let mut score = 0;
    let height = map.len() as u8;
    let width = map.first().unwrap().len() as u8;

    for (x,y) in starts{
        let mut open:VecDeque<(Coord, Coord)> = VecDeque::new();

        add_directions2(&mut open,(*x, *y), width, height);

        while !open.is_empty(){
            let ((dx, dy), (ox, oy)) = open.pop_front().expect("Critical error. No value in open");  

            let value = map[dy as usize][dx as usize];
            let from = map[oy as usize][ox as usize];
            let dif = value as i8 - from as i8;

            if dif == 1{
                if value == 9 {score += 1; continue;}
                else{
                    add_directions2(&mut open,(dx, dy), width, height);
                }
            }
            else{continue;}
        }
    }
    score
}

fn part1((map, starts) : &(Map, HashSet<Coord>)) -> u32{
    let mut score = 0;
    let height = map.len() as u8;
    let width = map.first().unwrap().len() as u8;

    for (x,y) in starts{
        let mut open:VecDeque<(Coord, u8)> = VecDeque::new();
        let mut close:HashSet<Coord> = HashSet::new();

        add_directions(&mut open,*x,*y, width, height, 0);

        while !open.is_empty(){
            let ((x, y), from) = open.pop_front().expect("Critical error. No value in open");  
            if close.contains(&(x,y)) {continue;}

            let value = map[y as usize][x as usize];
            let dif = value as i8 - from as i8;

            if dif == 1{
                if value == 9 {score += 1; close.insert((x,y)); continue;}
                else{
                    add_directions(&mut open,x, y, width, height, value);
                    close.insert((x,y));
                }
            }
            else{continue;}
        }
    }
    score
}
fn add_directions2(open: &mut VecDeque<(Coord, Coord)>, (ox, oy): Coord, width: u8, height: u8) {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(|(rx, ry)| {
            let new_x = (ox as i16).checked_add(rx)?; 
            let new_y = (oy as i16).checked_add(ry)?; 
            (new_x >= 0 && new_x < width as i16 && new_y >= 0 && new_y < height as i16)
                .then_some((new_x as u8, new_y as u8))
        })
        .for_each(|coord| open.push_back((coord, (ox, oy))));
}

fn add_directions(open: &mut VecDeque<(Coord, u8)>, x: u8, y: u8, width: u8, height: u8, from: u8) {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(|(dx, dy)| {
            let new_x = (x as i16).checked_add(dx)?; 
            let new_y = (y as i16).checked_add(dy)?; 
            (new_x >= 0 && new_x < width as i16 && new_y >= 0 && new_y < height as i16)
                .then_some((new_x as u8, new_y as u8))
        })
        .for_each(|coord| open.push_back((coord, from)));
}

fn parse(input : &str) -> (Map, HashSet<Coord>){
    let mut map = Vec::new();
    let mut starts = HashSet::new();

    for (j, line) in input.lines().enumerate(){
        let mut tiles = Vec::new();
        for (i,c) in line.chars().enumerate(){
            tiles.push(c.to_digit(10).unwrap_or(255) as u8);
            
            if c == '0' {starts.insert((i as u8,j as u8));}
        }
        map.push(tiles);
    }

    (map, starts)
}


#[cfg(test)]
mod test{
    use std::{collections::HashSet, fs, vec};

    use crate::{parse, part1, part2};


    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(parse(&text), (vec![vec![8, 9, 0, 1, 0, 1, 2, 3], vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5], vec![9, 6, 5, 4, 9, 8, 7, 4], vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2], vec![0, 1, 3, 2, 9, 8, 0, 1], vec![1, 0, 4, 5, 6, 7, 3, 2]],
            HashSet::from([(4,2), (2,0), (0,6), (6,4), (6,6), (1,7), (2,5), (5,5), (4,0)])));
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part1(&parse(&text)), 36);
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part2(&parse(&text)), 81);
    }

}