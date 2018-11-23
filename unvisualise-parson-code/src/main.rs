/**
* Title: Unvisualise Parson's Code
* Author: W.K.S
* Description: http://codegolf.stackexchange.com/questions/68006/unvisualize-parsons-code
*/
use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {

     let contour = read().unwrap();
     let mut position_line : Vec<(usize,char)> = Vec::new();
     let lines : Vec<&str>= contour.lines().collect();
     for line in lines.iter() {
        for (j,c) in line.chars().enumerate(){
            if c == '/' || c == '\\' || c=='-' {
                position_line.push((j,c));
            }
        }
     }

     position_line.sort_by(|pl1,pl2| {
        pl1.0.cmp(&pl2.0)
     });

     for (i,pl) in position_line.iter().enumerate(){
        if i == 0 {print!("*")}
        
        if pl.1 == '\\'{
            print!("D")
        }else if pl.1 == '/'{
            print!("U")
        }else{
            print!("R")
        }
        
     }

     print!("\n")
}

fn read()->Result<String, io::Error>{
	let mut f = try!(File::open("contour"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}