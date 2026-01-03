use std::{cmp, fs};
use std::time::Instant;

type Block = (u16, u16); //Restar 1 para la ID real

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();

    let t = Instant::now();
    let input = parse(&text);
    let r1 = part1(&mut input.clone());
    println!("Resultado de la parte 1: {}\nObtenida en {} ms", r1, t.elapsed().as_millis());

    let t = Instant::now();
    let r2 = part2(&mut parse(&text));
    println!("Resultado de la parte 2: {}\nObtenida en {} ms", r2, t.elapsed().as_millis());
}

fn part2(input : &mut Vec<Block>) -> usize{
    let mut c = input.clone();
    let temp = change_input(&mut c);
    *input = temp;

    for block in input.iter(){
        if block.1 == 0{print!("{}", format!(".").repeat(block.0 as usize));} else {print!("{}", format!("{}",block.1-1).repeat(block.0 as usize));}
    }

    let mut checksum = 0;
    let mut i = 0;
    for (j, v) in input.iter(){
        if *v == 0 {break;}
        else {
            for value in 0..*j{
                checksum += (i + value) as usize * (*v as usize - 1);
            }
            i += *j;
        }
    }
    checksum
}

fn change_input(input : &mut Vec<Block>) -> Vec<Block> {
    let mut next = input.clone();
    
    println!("{:?}", &next);
    for (j, blockw) in input.iter().enumerate().rev(){
        if blockw.1 == 0{
            for (i, block) in input.iter().enumerate(){
                if block.1 > 0 && j > i{
                    let mut ei = i;
                    let mut wi = j;
                    let mut wiID = 0;
                    let change;
                    if input.get(wi).unwrap().0 <= input.get(ei).unwrap().0{
                        change = input.get(wi).unwrap().0; 
                    }
                    else{continue;}
                    
                    if change == input.get(ei).unwrap().0{
                        next.swap(ei, wi);
                    }
                    else{
                        let writeblock = next.get_mut(wi).unwrap(); 
                        wiID = writeblock.1;
                        if writeblock.0 - change == 0 {
                            next.remove(wi);
                            wi -= 1;
                        }  
                        else {
                            writeblock.0 -= change;
                        }
                        
                        let emptyblock = next.get_mut(ei).unwrap(); 
                        if emptyblock.0 - change == 0 {
                            next.remove(ei);
                            ei -= 1;
                            wi -= 1;
                        }  
                        else {
                            emptyblock.0 -= change;
                        }
                        
                        next.insert(ei, (change, wiID));
                        next.insert(wi+1, (change, 0));
                        
                    }
                    println!("{:?}", &next);
                    println!("----");
                }
            }
        }
    }
    return input.clone();
}

fn part1(input : &mut Vec<Block>) -> usize{
    while !is_compacted(&input){
        let mut empty_index = None;

        let mut written_index = None;

        for (i, block) in input.iter().enumerate(){
            if block.1 == 0 {empty_index = Some(i); break;}
        }

        for (i, block) in input.iter().enumerate().rev(){
            if block.1 > 0 {written_index = Some(i); break;}
        }

        if let (Some(ei), Some(mut wi)) = (empty_index, written_index){
            let change = cmp::min(input.get(wi).unwrap().0, input.get(ei).unwrap().0);

            if let Some(written) = input.get_mut(wi) {
                written.0 -= change;
            }

            if let Some(empty) = input.get_mut(ei) {
                empty.0 -= change;
            }

            // Insertar el nuevo bloque después de modificar
            input.insert(ei, (change, input[wi].1));
            wi += 1;

            // Si el bloque vacío quedó en 0, lo eliminamos
            if input[ei + 1].0 == 0 {
                input.remove(ei + 1);
                wi -= 1;
            }

            if input[wi].0 == 0 {
                input.remove(wi);
            }
        }
    }
    
    let mut checksum = 0;
    let mut i = 0;
    for (j, v) in input.iter(){
        if *v == 0 {break;}
        else {
            for value in 0..*j{
                checksum += (i + value) as usize * (*v as usize - 1);
            }
            i += *j;
        }
    }
    checksum
}

fn is_compacted(input : &Vec<Block>) -> bool {
    let mut has_find_empty = false;

    for block in input{
        if block.1 != 0 && has_find_empty{
            return false;
        }
        else if block.1 == 0 && !has_find_empty{
            has_find_empty = true;
        }
    }
    return true;
}

fn parse(input : &str) -> Vec<Block>{
    let mut bloques:Vec<Block> = Vec::new();
    let mut is_empty = false;
    let mut id = 0;

    for char in input.chars(){
        let num = char.to_digit(10).expect("Expected value") as u16; 
        
        if is_empty && num > 0{
            bloques.push((num, 0));
            is_empty = false;
        } 
        else if num > 0 {
            bloques.push((num, id + 1));
            id += 1;
            is_empty = true;  
        }
        else {
            is_empty = !is_empty;
        }
    }

    bloques
}

#[cfg(test)]
mod test{
    use std::fs;
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse(){
        let text = fs::read_to_string("inputTest.txt").unwrap();

        assert_eq!(parse(&text), vec![
            (1, 1), (2, 0), (3, 2), (4, 0), (5, 3)
        ]);
    }

    #[test]
    fn test_part1(){
        let text = fs::read_to_string("inputTest.txt").unwrap();

        assert_eq!(part1(&mut parse(&text)),1928);
    }

    #[test]
    fn test_part2(){
        let text = fs::read_to_string("inputTest.txt").unwrap();

        assert_eq!(part2(&mut parse(&text)),2858);
    }

}