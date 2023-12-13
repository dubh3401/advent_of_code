use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

#[path = "./cube_bag.rs"]
mod cube_bag;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve()->() {
    // let out_file: &Path = Path::new("src/advent_d2/puzzle_output.txt");

    // if Path::new(out_file).exists(){
    //     remove_file(out_file).expect("error in deletion");
    // }
    let mut result:u32 = 0;
    if let Ok(lines) = read_lines("src/advent_d2/puzzle_input.txt") {
        for line in lines {
            if let Ok(value) = line{
                let game:cube_bag::Game = cube_bag::Game::new(&value);
                println!("id:{:?} is {:?} ",game.get_id(), if game.is_valid() {"valid"} else {"is not valid"});
                if game.is_valid(){
                    // result += game.get_id() as u16;
                    result += game.get_power();
                }
            }
        }
    }
    println!("total is: {}",result);
}