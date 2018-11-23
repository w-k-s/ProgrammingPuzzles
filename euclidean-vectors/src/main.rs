/**
* Title: Euclidean Vectors
* Author: W.K.S
* Description: http://codegolf.stackexchange.com/questions/68867/euclidean-vectors
*/
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn read_file(file_name: &str)->Result<String,Error>{

    let mut file = try!(File::open(file_name));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn main(){

    let vector_diagram = match read_file("vector"){
        Ok(content)=>content,
        Err(err)=>panic!(err)
    };

    
    let mut line_length = 0usize;
    {
        let lines : Vec<&str> = vector_diagram.lines().collect();

        for (i,line) in lines.iter().enumerate(){
            if line_length == 0{
                //increment line_length by one to account for line breaks.
                line_length = line.len()+1       
            }else if line.len() != line_length-1{
                panic!("All lines in vector diagram must be the same length. Line {} has different length.",i+1);
            }
        }
    }

    let mut origin = (0f64,0f64);
    let mut point = (0f64,0f64);

    for (i,c) in vector_diagram.chars().enumerate(){

        //multiply y coordinates by -1 because computer screens count from top->bottom.

        if c == 'x'{
            origin = ((i%line_length) as f64,(i/line_length)as f64 * -1f64);
        }else if is_junction_char(c) && num_lines_leading_to_junction(i,line_length,&vector_diagram)==1{
            //turning-points in the vector diagram will have two connecting lines at a junction.
            //end-point will only have one line leading up to the junction.
            point = ((i%line_length) as f64,(i/line_length)as f64 * -1f64);
        }   
    }
 
    let displacement = ((origin.1 - point.1).powi(2)+(origin.0 - point.0).powi(2)).sqrt();
    let angle : f64= match (point.1- origin.1).atan2(point.0 - origin.0).to_degrees(){
        neg_angle if neg_angle < 0f64 => neg_angle + 360f64,
        angle => angle,
    };

    let direction = match angle as i32 {
        0 => "E",
        1...89 => "NE",
        90 => "N",
        91...179 => "NW",
        180 => "W",
        181...269 => "SW",
        270 => "S",
        271...359 => "SE",
        _ => "-",
    };
    
    print!("In:\n{}\n\n",vector_diagram);
    //TODO Significant figures.
    print!("Out:\n{:.1} units @ {:.1} degrees {}\n\n",displacement,angle,direction);
}

// 'v','^','<' and '>' represent junctions in the vector diagram.
fn is_junction_char(c : char)->bool{
    c == '<' || c == '^' || c == '>' || c == 'v'
}

fn is_connecting_line(c: char)->bool{
    c == '|' || c=='-'
}

//This function returns the number of lines that connect to a junction at position 'pos'
fn num_lines_leading_to_junction(pos : usize,line_length : usize, vector_diagram : &str)->usize{
    let mut num_lines = 0usize;
    let diagram_chars : Vec<char>= vector_diagram.chars().collect();
    {
        let mut increment_if_connecting_line_at = |char_at_pos: Option<&char>|{
            match char_at_pos {
                Some(c) if is_connecting_line(*c)=> {
                    num_lines +=  1;
                },
                _ =>{},
            }
        };

        increment_if_connecting_line_at(diagram_chars.get(pos + 1));
        increment_if_connecting_line_at(diagram_chars.get(pos + line_length));
        if pos >= 1 {
            increment_if_connecting_line_at(diagram_chars.get(pos - 1));
        }
        if pos >= line_length{
            increment_if_connecting_line_at(diagram_chars.get(pos - line_length))
        }
    }

    num_lines  
}