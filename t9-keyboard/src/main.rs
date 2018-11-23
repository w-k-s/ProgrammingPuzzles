#[macro_use]
extern crate lazy_static;

use std::env;
use std::collections::HashMap;

lazy_static! {
    
    static ref T9 : HashMap<char,Vec<char>> = {
        
        let mut t9 = HashMap::new();
        t9.insert('1',vec!['.','?','!','1']);
        t9.insert('2',vec!['A','B','C','2']);
        t9.insert('3',vec!['D','E','F','3']);
        t9.insert('4',vec!['G','H','I','4']);
        t9.insert('5',vec!['J','K','L','5']);
        t9.insert('6',vec!['M','N','O','6']);
        t9.insert('7',vec!['P','Q','R','S','7']);
        t9.insert('8',vec!['T','U','V','8']);
        t9.insert('9',vec!['W','X','Y','Z','9']);
        t9.insert('0',vec![' ']);
        
        t9
    };
}

fn main() {
    
    let mut t9_text = env::args().nth(1).expect("Usage:\n ./t9 <T9 TEXT>#");
    
    //T9 Text needs to be suffixed with # to ensure the last sequence of recurring digits is printed.
    t9_text.push('#');
    let t9_text = t9_text;
    
    let mut decoded_text = String::from("");
    let mut recurring_char = '\0';
    let mut recurring_char_count = 0;
    
    for a_char in t9_text.chars() {
        
        if a_char == recurring_char {
            
            recurring_char_count = recurring_char_count + 1;
            
        }else if a_char == '*' && recurring_char_count > 0{
           
            recurring_char_count = recurring_char_count - 1;
            recurring_char = '\0';
            
        }else{
            
            if let Some(t9_char) = T9.get(&recurring_char)
                .and_then(|letters| letters.get(recurring_char_count).cloned()) {
                decoded_text.push(t9_char)
            }
            
            recurring_char = a_char;
            recurring_char_count = 0;
            
        }
        
        //println!("Current Character: {}, Recurring Character: {}, Decoded: {}", a_char, recurring_char,decoded_text);
    }
    
    print!("Decoded Text: {:?}\n", decoded_text);
}