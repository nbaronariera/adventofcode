use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Coord = (u8, u8);
type Area = (HashSet<Coord>, char);


#[derive(PartialEq, Eq, Clone, Debug)]
struct Map{
    map: Vec<Vec<char>>,
    areas: Vec<Area>,
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    println!("Datos parseados en {} ms", t.elapsed().as_millis());

    let t = Instant::now();
    let r1 = part1(&input);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1.1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(r1.0, (input.map[0].len() as u8, input.map.len() as u8));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}


fn get_ordered_perimeter(perimeter: &HashSet<Coord>, width: u8, height: u8) -> Vec<Coord> {
    let &start = perimeter.iter().min_by_key(|&&(x, y)| (y, x)).unwrap();
    let mut ordered = vec![start];
    let mut current = start;
    let mut previous: Option<Coord> = None;

    while ordered.len() < perimeter.len() {
        let neighbors: Vec<Coord> = add_directions(current.0, current.1, width, height)
            .into_iter()
            .flatten()  // Descarta los None
            .filter(|coord| perimeter.contains(coord))
            .collect();

        let next = neighbors.into_iter().find(|&n| Some(n) != previous && !ordered.contains(&n));

        if let Some(n) = next {
            ordered.push(n);
            previous = Some(current);
            current = n;
        } else {
            break;
        }
    }
    ordered
}

fn count_turns(ordered: &Vec<Coord>) -> usize {
    0
}

fn part2(borders: Vec<Area>, (width, height): Coord) -> usize{
    let mut sections = 0;

    for area in borders{
        let mut temp = area.0.clone();
        let surface = area.0.len();
        let mut changes = 0;
        
        while temp.len() > 0 {
            let ordered_perimeter = get_ordered_perimeter(&temp, width, height);
            
            for coord in &ordered_perimeter{
                temp.remove(&coord);
            }

            changes += count_turns(&ordered_perimeter);
        }

        sections += surface * changes;
    }

    sections
}

fn part1(input : &Map) -> (Vec<Area>, usize) {
    let mut count = 0;
    let map = &input.map;
    let areas = &input.areas;
    let mut area_borders:Vec<Area> = Vec::new();

    for (area, char) in areas.iter(){
        let space = area.len();
        let  mut perimeter = 0;
        let mut perimeter_coord = HashSet::new();

        for coord in area {
            let touching = add_directions(coord.0 as u8, coord.1 as u8, map[0].len() as u8, map.len() as u8);

            for possible in touching{
                match possible {
                    None => {perimeter += 1; perimeter_coord.insert(*coord);}
                    Some(coor) if !area.contains(&coor) => {perimeter += 1; perimeter_coord.insert(*coord);}
                    _ => {}
                };
            }
        }

        area_borders.push((perimeter_coord, *char));
        count += space * perimeter;
    }    
    (area_borders, count)
}

fn parse(input : &str) -> Map{
    let mut raw: Vec<Vec<char>> = Vec::new();
    let mut unvisited_coords:Vec<Coord> = Vec::new();

    for (y, line) in input.lines().enumerate(){
        let mut temp = Vec::new();
        for (x, c) in line.chars().enumerate(){
            temp.push(c);
            unvisited_coords.push((x as u8,y as u8)); 
        }
        raw.push(temp);
    }

    let map = raw;
    let rows = map.len() as u8;
    let cols = map[0].len() as u8;
    let mut areas:Vec<Area> = Vec::new();

    while let Some(&(x, y)) = unvisited_coords.first(){
        let mut area: HashSet<(u8, u8)> = HashSet::new(); 
        let char = map[y as usize][x as usize];

        area.insert((x, y),);
        unvisited_coords.remove(0);

        let mut position = add_directions(x, y, rows, cols);

        while let Some(poss_coord) = position.pop(){
            match poss_coord {
                None => {}    
                Some((px, py)) => {
                    if !unvisited_coords.contains(&(px, py)) {continue;}
                    else if map[py as usize][px as usize] == char {
                        area.insert((px, py));
                        unvisited_coords.remove(unvisited_coords.iter().position(|(ox, oy)| *ox == px && *oy == py ).unwrap());

                        position.append(&mut add_directions(px, py, rows, cols));
                    }
                }
            }
        }

        areas.push((area, char));
    }

    Map{map, areas}
}

fn add_directions(x: u8, y: u8, width: u8, height: u8) -> Vec<Option<Coord>> {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    directions
        .iter()
        .map(|&(dx, dy)| {
            let new_x = (x as i16).checked_add(dx);
            let new_y = (y as i16).checked_add(dy);
            if let (Some(nx), Some(ny)) = (new_x, new_y) {
                if nx >= 0 && nx < width as i16 && ny >= 0 && ny < height as i16 {
                    Some((nx as u8, ny as u8))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
#[cfg(test)]
mod test{
    use std::fs;

    use crate::{parse, part1, Map};

    #[test]
    fn test_parse(){
        let text = "AAAA\nBBCD\nBBCC\nEEEC";

        assert_eq!(parse(&text), Map{map:vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C']],
            areas: vec![
                (vec![(3, 0), (0, 0), (1, 0), (2, 0)].into_iter().collect(), 'A'),
                (vec![(0u8, 1u8), (1, 1), (0, 2), (1, 2)].into_iter().collect(), 'B'),
                (vec![(3, 2), (2, 2), (3, 3), (2, 1)].into_iter().collect(), 'C'),
                (vec![(3,1)].into_iter().collect(), 'D'),
                (vec![(1, 3), (2, 3), (0, 3)].into_iter().collect(), 'E') 
            ]});
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part1(&parse(&text)).1, 1930);
    }

    #[test]
    fn test_part2(){
    }

}