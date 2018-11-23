/*
* Title: Undress a String
* Description: https://codegolf.stackexchange.com/questions/143376/undress-a-string
*/
fn main() {
    let mut str = "     codegolf     ";
    let mut count = 0u8;
    
    print!("{}\n",str);
    while str.starts_with(" ") || str.ends_with(" ") {
        count += 1;
        if count%2 != 0 || !str.ends_with(" ") {
            str = &str[1..];
        }else if count%2 == 0|| !str.starts_with(" ") {
            str = &str[..(str.len()-1)];
        }
        print!("{}\n",str);
    }
}