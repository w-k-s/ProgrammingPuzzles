/**
*Match Adjacent Words
*Author: W.K.S.
*Description: http://codegolf.stackexchange.com/questions/70092/matching-adjacent-words
*/
extern crate time;

use std::ops::Sub;

fn main(){
	let start_time = time::now();

	assert_eq!(true,are_adjacent("A","B"));
	assert_eq!(true,are_adjacent("A","B"));
	assert_eq!(true,are_adjacent("C","B"));
	assert_eq!(true,are_adjacent("DD","CE"));
	assert_eq!(true,are_adjacent("DE","FC"));
	assert_eq!(true,are_adjacent("ABCD","BCDE"));
	assert_eq!(true,are_adjacent("AACC","DBBB"));
	assert_eq!(true,are_adjacent("DJENSKE","FDJCLMT"));
	assert_eq!(true,are_adjacent("DEFGHIJKL","HJLEHMCHE"));
	assert_eq!(true,are_adjacent("IKLIJJLIJKKL","LJLJLJLJLJHI"));
	assert_eq!(true,are_adjacent("ACEGIKMOQSUWY","BLNPRDFTVHXJZ"));
	assert_eq!(true,are_adjacent("QQSQQRRQSTTUQQRRRS","PQTTPPTTQTPQPPQRTP"));
	assert_eq!(true,are_adjacent("ELKNSDUUUELSKJFESD","DKJELKNSUELSDUFEUS"));

	assert_eq!(false,are_adjacent("A","C"));
	assert_eq!(false,are_adjacent("A","Z"));
	assert_eq!(false,are_adjacent("B","J"));
	assert_eq!(false,are_adjacent("JK","J"));
	assert_eq!(false,are_adjacent("CC","BA"));
	assert_eq!(false,are_adjacent("CE","D"));
	assert_eq!(false,are_adjacent("DJENSKE","GDJCLMT"));
	assert_eq!(false,are_adjacent("DEFGHIJKL","HJLHMCHE"));
	assert_eq!(false,are_adjacent("IJKLIJKLKIJL","LIJLLHJLJLLL"));
	assert_eq!(false,are_adjacent("AWSUKMEGICOQY","RSHXBLJLNQDFZ"));
	assert_eq!(false,are_adjacent("QQSQQRRQSTTUQQQRRS","PQTTPPTTQTPQPPQRTT"));
	assert_eq!(false,are_adjacent("QQSQQRRQSTTUQQQRRS","PQTTPPTTQTPQPPQRTT"));
	assert_eq!(false,are_adjacent("ELKNSDUVWELSKJFESD","DKJELKNSUELSDUFEUS"));

	let duration = time::now().sub(start_time);
	println!("Total time: {} milliseconds",duration.num_milliseconds());
}

fn are_adjacent(w: &str, x: &str)->bool{

    if w.len() == x.len(){

        return {

            let mut c : Vec<char> = w.chars().collect();
            let mut d : Vec<char> = x.chars().collect();

            c.sort();
            d.sort();

            for (e,f) in c.iter().zip(d.iter()){
                if (((*e as u8) as f64) - ((*f as u8) as f64)).abs() > 1f64{
                    return false
                } 
            }

            true
        }
    }

    false
}