use substring::Substring;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;


const RED_MAX_COUNT:u8 = 12;
const GREEN_MAX_COUNT:u8 = 13;
const BLUE_MAX_COUNT:u8 = 14;


struct Game {
    red_count:u8,
    green_count:u8,
    blue_count:u8,
    id:u8
}

impl Game{
    // parse game -> nb_red, nb_green, nb_blue, id
    fn new(line:&String) -> Game{
        let id_str:String = String::from("Game ");
        let red_str:String = String::from(" red");
        let green_str:String = String::from(" green");
        let blue_str:String = String::from(" blue");

        //find game id
        let id:u8 = String::from(line.substring(
            line.find(&id_str).expect("id delimiter not found")+id_str.len(),
            line.find(':').expect("pattern not found")))
            .parse::<u8>().expect("not a number");
       
        let sub_string= String::from(line.substring(0, line.find(':').expect("pattern not found")));
        
        let red:u8 = String::from(sub_string.substring(
            sub_string.find(" ").expect("pattern not found")+1,
            sub_string.find(&red_str).expect("id delimiter not found")-1))
            .parse::<u8>().expect("not a number");
            
        
        
        return Game{red_count:red,green_count:0,blue_count:0,id:id};
    }
    // is game valid
}


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

    if let Ok(lines) = read_lines("src/advent_d2/puzzle_input.txt") {
        for line in lines {
            if let Ok(value) = line{
                println!("{:?}",Game::new(&value).id);
            }
        }
    }
}