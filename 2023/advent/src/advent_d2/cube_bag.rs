use substring::Substring;
use regex::Regex;

const RED_MAX_COUNT:u8 = 12;
const GREEN_MAX_COUNT:u8 = 13;
const BLUE_MAX_COUNT:u8 = 14;

const ID_STR:&str ="Game ";
static RED_STR:&str =" red";
static GREEN_STR:&str =" green";
static BLUE_STR:&str =" blue";



pub struct Game {
    red_count:u8,
    green_count:u8,
    blue_count:u8,
    id:u8,
    valid:bool
    
}

impl Game{
    // parse game -> nb_red, nb_green, nb_blue, id
    pub fn new(line:&String) -> Game{

        println!("{}",line);

        let mut red:u8 = 0;
        let mut green:u8 = 0;
        let mut blue:u8 = 0;
        let mut validity:bool = true;
        //find game id
        let s_game = line.find(&ID_STR).expect("id delimiter not found");
        let e_game = line.find(':').expect("pattern not found");
        let id:u8 = String::from(line.substring(
            s_game+ID_STR.len(),
            e_game))
            .parse::<u8>().expect("not a number");
        
        let sub_string= String::from(line.substring(e_game+1,line.len()));


        //parse the rest as it only contains colors
        let pattern = Regex::new(r",|;").expect("wrong pattern");
        for part in pattern.split(&sub_string.as_str()) {
            println!(">{}<", part);
            let draw = String::from(part);
            
            let mut color_asked = RED_STR;
            if draw.contains(color_asked){
                let val = _color_count(&draw,color_asked);
                
                if (val > red) && is_draw_valid(val,color_asked) {
                    red = val;
                }
                
                if !is_draw_valid(val, color_asked){
                    validity = false;
                }
            
            }  

            color_asked = GREEN_STR;
            if draw.contains(color_asked){
                let val = _color_count(&draw,color_asked);
                
                if (val > green) && is_draw_valid(val,color_asked) {
                    green = val;
                }
                
                if !is_draw_valid(val, color_asked){
                    validity = false;
                }
            
            }  

            color_asked = BLUE_STR;
            if draw.contains(color_asked){
                let val = _color_count(&draw,color_asked);
                
                if (val > blue) && is_draw_valid(val,color_asked) {
                    blue = val;
                }
                
                if !is_draw_valid(val, color_asked){
                    validity = false;
                }
            }  

        }
        
        return Game{red_count:red,green_count:green,blue_count:blue,id:id,valid:validity};
    }



    pub fn is_valid(&self) -> bool {
        return self.valid;
    }

    pub fn get_id(&self) -> u8 {
        return self.id;
    }

    pub fn get_power(&self) -> u32{
        return (self.red_count as u32) * (self.green_count as u32) * (self.blue_count as u32);
    }
    
}


fn _color_count(draw:&String,desired_color_str:&str)-> u8{
    let s_color = draw.find(String::from(desired_color_str).as_str()).unwrap();
    let val:u8 = String::from(draw.as_str().substring(1,s_color)).parse::<u8>().expect("not a number");
    return val;
}

fn is_draw_valid(val:u8,desired_color_str:&str)->bool{
    let mut ret:bool = false;
    if desired_color_str == RED_STR {
        ret = (val <= RED_MAX_COUNT) && (val > 0);
    }

    if desired_color_str == GREEN_STR {
        ret = (val <= GREEN_MAX_COUNT) && (val > 0);
    }

    if desired_color_str == BLUE_STR {
        ret = (val <= BLUE_MAX_COUNT) && (val > 0);
    }

    println!("{}",ret);

    ret = true;
    return ret;
}