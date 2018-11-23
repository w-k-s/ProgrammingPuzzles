/**
* Title: You're on a 8 day streak!
* Author: W.K.S
* Description: http://codegolf.stackexchange.com/questions/66841/youre-on-a-8-day-streak
*/
extern crate rand;

use rand::Rng;

fn main() {
    let quantity = rand::thread_rng().gen_range(0, 1000000);

    println!("You're on {} {} streak",indefinite_article(quantity),quantity)
}

fn indefinite_article(mut num: i32)->String{
	
    while num > 1000 {
    	num = num / 1000;
    }

    let string_num = num.to_string();
    
    return (if string_num.starts_with("8")
    	|| string_num.len()==2 && (string_num.starts_with("11") || string_num.starts_with("18")){
    	"an"
    }else{
    	"a"
    }).to_string();
}

#[test]
fn test_indefinite_article(){
	
	assert_eq!(indefinite_article(0),"a");
	assert_eq!(indefinite_article(8),"an");
	assert_eq!(indefinite_article(11),"an");
	assert_eq!(indefinite_article(84),"an");

	assert_eq!(indefinite_article(1_111),"a");
	assert_eq!(indefinite_article(1_863),"a");
	assert_eq!(indefinite_article(8_192),"an");

	assert_eq!(indefinite_article(11_000),"an");
	assert_eq!(indefinite_article(18_000),"an");
	assert_eq!(indefinite_article(81_000),"an");

	assert_eq!(indefinite_article(110_000),"a");
	assert_eq!(indefinite_article(180_000),"a");

	assert_eq!(indefinite_article(1_141_592),"a");
	assert_eq!(indefinite_article(1_141_592),"a");
	assert_eq!(indefinite_article(1_897_932),"a");

	assert_eq!(indefinite_article(11_234_567),"an");
	assert_eq!(indefinite_article(18_675_369),"an");

}