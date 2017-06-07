extern crate connectfour;

use std::io::stdin;
use connectfour::connectfour::*;

fn main() {
	let mut board = Board::new();
	
	loop {
		println!("Player 1, choose a column to drop into.");
		println!("{}", board);
		
		loop {
			let p1_choice = get_input();
			
			match convert_to_int(&p1_choice) {
				Ok(column) => {
					if !(column > 0 && column < 8) {
						println!("There are only columns 1 to 7.");
						continue;
					}
					
					if !board.drop_into(column as usize, SlotState::Red) {
						println!("This column is full! Drop into another one.");
						continue;
					}
				},
				
				Err(e) => {
					println!("Invalid input! {}", e);
					continue;
				},
			};
			
			break;
		}
		
		if board.check_for_winner() {
			println!("Player 1 won!");
			break;
		}
		
		println!("Player 2, choose a column to drop into.");
		println!("{}", board);
		
		loop {
			let p2_choice = get_input();
			
			match convert_to_int(&p2_choice) {
				Ok(column) => {
					if !(column > 0 && column < 8) {
						println!("There are only columns 1 to 7.");
						continue;
					}
					
					if !board.drop_into(column as usize, SlotState::Blue) {
						println!("This column is full! Drop into another one.");
						continue;
					}
				},
				
				Err(e) => {
					println!("Invalid input! {}", e);
					continue;
				},
			};
			
			break;
		}
		
		if board.check_for_winner() {
			println!("Player 2 won!");
			break;
		}
	}
	
	println!("{}", board);
}

fn convert_to_int(s: &String) -> Result<i32, &str> {
	if s.chars().any(|c| !c.is_numeric()) {
		return Err("String contains non numeric digits.");
	}
	
	let n: u32 = s.chars().rev().enumerate().map(|(i, x)| (i, x.to_digit(10).unwrap())).fold(0, |acc, (i, x)| acc + (10u32.pow(i as u32) * x));
	
	Ok(n as i32)
}

fn get_input() -> String {
	let mut s = String::new();
	
	stdin().read_line(&mut s).expect("Failed to read input.");
	
	s = s.lines().next().unwrap().to_string();
	
	s
}