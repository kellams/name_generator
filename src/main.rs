/* Asks user for number of syllables and number of words to generate
 * then generates words based on the given values.
 * 
 * Example:
 * How many syllables?
 * 3
 * How many words?
 * 2
 * celera
 * nesico
 */

extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	
	//initialize the array
	
	let syllables = initialize();
	
	//get number of syllables
	
	println!("How many syllables?");
		
	let mut number_of_syllables = String::new();
		
	io::stdin().read_line(&mut number_of_syllables)
		.expect("Failed to read line");
	
	let mut number_of_syllables_int: u32 = number_of_syllables.trim().parse()
		.expect("Could not read number");
	
	//get number of words
	
	println!("How many words?");
		
	let mut number_of_words = String::new();
		
	io::stdin().read_line(&mut number_of_words)
		.expect("Failed to read line");
	
	let mut number_of_words_int: u32 = number_of_words.trim().parse()
		.expect("Could not read number");
	
	//create immutable number of syllables
	
	let original_number_of_syllables = number_of_syllables_int;
	
	//create the words
		
	while number_of_words_int > 0 {
		while number_of_syllables_int > 0 {
			let random_number = rand::thread_rng().gen_range(0, 126);
			print!("{}", syllables[random_number]);
			number_of_syllables_int = number_of_syllables_int - 1;
		}
		number_of_syllables_int = original_number_of_syllables;
		number_of_words_int = number_of_words_int - 1;
		println!("");
	}
}

//initialize syllables function

fn initialize() -> Vec<String> {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm',
				'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    let mut x = 0;
    let mut syllables: Vec<String> = vec![String::new(); 126];
    
    
    for cons in consonants.iter() {
		for vowel in vowels.iter() {
			syllables[x] = cons.to_string() + &vowel.to_string();
			x = x + 1;
		}
	}
	syllables
}
