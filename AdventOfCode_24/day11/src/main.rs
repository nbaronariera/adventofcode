use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Stone = u64;
type Count = u64;

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&input, 25);
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part1(&input, 75);
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part1(input : &HashMap<Stone, Count>, times:u8) -> usize{
    let mut next = input.clone();

    for _ in 0..times{
        let mut temp = HashMap::new();
        
        for &stone in next.keys(){
            let value = next.get(&stone).unwrap();
            if stone == 0 {
                temp.entry(1).and_modify(|x| *x += value).or_insert(*value);
            }
            else if count_digits(stone) % 2 == 0 {
                let k = count_digits(stone) / 2;
                let divisor = 10u64.pow(k as u32) as Stone;
                let h1 = stone / divisor;
                let h2 = stone % divisor;

                temp.entry(h1).and_modify(|x| *x += value).or_insert(*value);
                temp.entry(h2).and_modify(|x| *x += value).or_insert(*value);
            }
            else {
                temp.entry(stone*2024).and_modify(|x| *x += value).or_insert(*value);
            }
        }
        next = temp;
    } 
    next.values().map(|x| *x as usize).sum()
}

fn count_digits(mut n: Stone) -> usize {
    if n == 0 { return 1; }
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

fn parse(input : &str) -> HashMap<Stone, Count>{
    let mut stones:HashMap<Stone, Count> = HashMap::new();

    for part in input.split_ascii_whitespace(){
        let key = part.parse::<Stone>().unwrap();
        stones.entry(key).and_modify(|x| *x += 1).or_insert(1);
    }
    stones
}


#[cfg(test)]
mod test{
    use std::{collections::HashMap, fs};
    use crate::{parse, part1};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        let mut map = HashMap::new();
        map.insert(17, 1);
        map.insert(125, 1);
        assert_eq!(parse(&text), map);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();
        assert_eq!(part1(&parse(&text), 6), 22);
        assert_eq!(part1(&parse(&text), 25), 55312);
        assert_eq!(part1(&parse(&text), 75), 65601038650482);
    }
}