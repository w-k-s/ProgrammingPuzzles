static ALPHABET : &'static str= "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let original = "I wanted to eat a chicken burger tonight,\nbut i chose to stay healthy instead";
    print!("{}\n->\n{}",original,caesar_cipher(original.to_string(),-1))
}

fn caesar_cipher(text: String, displacement: i8) -> String{
    
    let mut ciphered : Vec<char>= Vec::new();
    for a_char in text.to_lowercase().chars(){
        if let Some(cipher_char) = displace_letter(a_char,displacement){
            ciphered.push(cipher_char);
        }else{
            ciphered.push(a_char);
        }   
    }
    
    ciphered.iter().cloned().collect()
}

fn displace_letter(original: char, displacement: i8)->Option<char>{
    
    if let Some(old_index) = ALPHABET.chars().position(|c| c == original){
        
        let new_index = match old_index as i8 + displacement {
            a if a > 26 => a%26,
            b if b < 0 => 26 + b%26,
            c => c
        };
        
        return ALPHABET.chars().nth(new_index as usize)
    }
    
    return None
}