fn main() {

    assert_eq!(rein_number(1),"1");
    assert_eq!(rein_number(2),"12");
    assert_eq!(rein_number(3),"123");
    assert_eq!(rein_number(7),"1234567");
    assert_eq!(rein_number(9),"123456789");
    assert_eq!(rein_number(10),"10123456789");
    assert_eq!(rein_number(15),"101111111223344556789");
    assert_eq!(rein_number(34),"10001111111111111222222222222223333333334444555666777888999");
    assert_eq!(rein_number(42),"100001111111111111122222222222222233333333333333444444455556666777788889999")
}

fn rein_number(digits : i32)->String{

    let mut result = String::new();
    
    for i in 1..digits+1{
        result.push_str(&i.to_string());
    }
    
    let mut c : Vec<char> = result.chars().collect();

    c.sort();
    
    let index_first_one = c.binary_search(&'1').unwrap();

    c.remove(index_first_one);
    c.insert(0,'1');
    c.into_iter().collect()

}