use std::fs::{File,OpenOptions, remove_file};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::char::from_digit;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_line<P>(filename: P, numbers:String) ->()
where P: AsRef<Path>, {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("cannot open file");

    f.write(numbers.as_bytes()).expect("error writing");
}

fn has_numerical_str(word:&String) -> Option<String>{
    let matches:[&str;9]= ["one","two","three","four","five","six","seven","eight","nine"];
    let mut as_number_str:String = String::new();
    let mut i:u32 = 0;
   
    for spelled_number in matches{
        if word.contains(spelled_number){
            as_number_str.push(from_digit(i+1,10).expect("not a number"));
        }
        i += 1;
    } 

    if !as_number_str.is_empty(){
        Some(as_number_str)
    }else{
        None
    }


}

fn find_numbers(line:&str) -> String {
    let mut numbers:String = String::new();
    let mut sub_str:String = String::new();

    for character in line.chars(){
        sub_str.push(character);

        if sub_str.len() > 5{ // cheat
            sub_str.remove(0);
        }

        let number_word:Option<String> = has_numerical_str(&sub_str);
        if let Some(number) = number_word{
            numbers.push_str(number.as_str());
        }
        
        if character.is_numeric(){
            numbers.push(character);
            sub_str.clear();
        }
    }


    let mut first_last:String = String::new();
    first_last.push(numbers.chars().nth(0).expect("no index"));
    first_last.push(numbers.chars().nth(numbers.len()-1).expect("no index"));
    return first_last;
}

pub fn solve(){
    let out_file: &Path = Path::new("src/advent_d1/puzzle_output.txt");

    if Path::new(out_file).exists(){
        remove_file(out_file).expect("error in deletion");
    }

    if let Ok(lines) = read_lines("src/advent_d1/puzzle_input.txt") {
        for line in lines {
            if let Ok(value) = line{
                write_line(out_file,find_numbers(&value));
                write_line(out_file,'\n'.to_string());
            }
        }
    }

    let mut sum:i32 = 0;
    if let Ok(lines) = read_lines(out_file){
        for line in lines {
            if let Ok(value) = line{
                sum += value.parse::<i32>().expect("not a number");
            }
        }
    }

    println!("done! sum is {}",sum);
}