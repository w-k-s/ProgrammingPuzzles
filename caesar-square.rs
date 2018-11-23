fn main() {
    //should remove whitespaces from 'original'
    //maybe need to buffer 'original' if length not square number.
    let original = "ALPHABETA";
    let gap = (original.len() as f64).sqrt() as usize;
    let mut index = 0 - gap;
    let mut result = String::new();
    
    loop  {
        index = index + gap;
        if index >= original.len(){
            index = index % original.len()+1;
        }
        if result.len() == original.len(){
            break;
        }
        
        result.push(original.chars().nth(index as usize).unwrap())
    }
    
    print!("Caesar's Square:\n{}\n->\n{}",original,result);
}