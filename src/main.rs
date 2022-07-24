use std::io;
use rand::Rng;

fn main() {
	let mut user_input = String::new();
	let mut number_found = false;
	let random_number = rand::thread_rng().gen_range(0..100);
	
    println!("Please enter a number and I will tell you of you are too high, low or guessed the number correctly");
	
	while !number_found {
		println!("Enter number:");
		io::stdin().read_line(&mut user_input).unwrap();
		println!("User input: {}", user_input);
		
		match user_input.trim().parse::<i32>() {
			Ok(_n) => {
				number_found = inspect_number(_n, random_number);
			},
			Err(_e) => {
				println!("You entered a non-number, {}", _e);
			},
		}
		user_input = String::new();
	}
}

//Inspects a number and prints information to the screen.
//Will return true if it found the number. Otherwise false.
fn inspect_number(input:i32, number_to_find:i32) -> bool
{
	let position:i32;
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	println!("Inspecting number");
	position = number_position(number_to_find, input);
	
	if position == -1 {
		println!("You are too low.");
		return false;
	} else if position == 1 {
		println!("You are too high");
		return false;
	} else {
		println!("You found the number!");
		println!("Number to find is: {}", number_to_find);
		return true;
	}
}

//Gets the position of a number relative to another number and tells you if it is:
//Lower (-1), Higher (1) or Equal (0)
fn number_position(number_to_inspect:i32, number_to_get_close_too:i32)->i32
{
	if number_to_get_close_too > number_to_inspect {
		return 1;
	}
	
	if number_to_get_close_too < number_to_inspect {
		return -1;
	}
	
	return 0;
}
