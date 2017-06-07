#[cfg(test)]
mod tests {
	use connectfour::*;

    #[test]
    fn board_with_one_item() {
		let mut board = Board::new();
		board.drop_into(2, SlotState::Red);
		assert!(!board.check_for_winner());
    }
	
	#[test]
	fn board_with_winning_row() {
		let mut board = Board::new();
		board.drop_into(1, SlotState::Red);
		board.drop_into(7, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		board.drop_into(6, SlotState::Blue);
		board.drop_into(3, SlotState::Red);
		board.drop_into(5, SlotState::Blue);
		board.drop_into(4, SlotState::Red);
		assert!(board.check_for_winner());
	}
	
	#[test]
	fn board_with_winning_column() {
		let mut board = Board::new();
		board.drop_into(1, SlotState::Red);
		board.drop_into(2, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		board.drop_into(1, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		board.drop_into(1, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		board.drop_into(1, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		assert!(board.check_for_winner());
	}
	
	#[test]
	fn board_with_winning_diagonal_below_mid() {
		let mut board = Board::new();
		board.drop_into(7, SlotState::Red);
		board.drop_into(7, SlotState::Blue);
		board.drop_into(7, SlotState::Red);
		board.drop_into(7, SlotState::Blue);
		board.drop_into(6, SlotState::Red);
		board.drop_into(6, SlotState::Blue);
		board.drop_into(1, SlotState::Red);
		board.drop_into(6, SlotState::Blue);
		board.drop_into(5, SlotState::Red);
		board.drop_into(5, SlotState::Blue);
		board.drop_into(1, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		assert!(board.check_for_winner());
	}
	
	#[test]
	fn board_with_winning_antidiagonal_above_mid() {
		let mut board = Board::new();
		board.drop_into(7, SlotState::Red);
		board.drop_into(6, SlotState::Blue);
		board.drop_into(6, SlotState::Red);
		board.drop_into(1, SlotState::Blue);
		board.drop_into(5, SlotState::Red);
		board.drop_into(5, SlotState::Blue);
		board.drop_into(5, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		board.drop_into(4, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		board.drop_into(4, SlotState::Red);
		assert!(board.check_for_winner());
	}
	
	#[test]
	fn board_with_gap_in_row() {
		let mut board = Board::new();
		board.drop_into(1, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		board.drop_into(2, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		board.drop_into(7, SlotState::Red);
		board.drop_into(4, SlotState::Blue);
		board.drop_into(6, SlotState::Red);
		board.drop_into(7, SlotState::Blue);
		board.drop_into(5, SlotState::Red);
		board.drop_into(6, SlotState::Blue);
		board.drop_into(3, SlotState::Red);
		assert!(!board.check_for_winner());
	}
}

pub mod connectfour {
	use std::fmt::{Display, Formatter, Result};

	pub struct Board {
		columns: [Column; 7],
		// Container, Slot tuple
		last_insert: (usize, usize),
	}
	
	impl Board {
		pub fn new() -> Self {
			Board {
				columns: [Column::new(), Column::new(), Column::new(), Column::new(), Column::new(), Column::new(), Column::new()],
				last_insert: (0, 0),
			}
		}
		
		pub fn drop_into(&mut self, column_number: usize, symbol: SlotState) -> bool {
			let mut first_available_slot = 0;
			
			if self.columns[column_number-1].slots.into_iter().rev().enumerate().any(|(i, s)| {first_available_slot = 5 - i; return s.state == SlotState::Empty;}) {
				self.columns[column_number-1].slots[first_available_slot].state = symbol;
				self.last_insert = (column_number-1, first_available_slot);
				return true;
			}
			
			false
			
		}
		
		pub fn check_for_winner(&self) -> bool {
			// Check the columns
			if self.columns[self.last_insert.0].contains_run_of_four() {
				return true;
			}
			
			// Check the rows
			let mut run = 1;
			let mut last_symbol = &SlotState::Empty;
			for j in 0..7 {
				let s = &self.columns[j].slots[self.last_insert.1];
				
				if last_symbol == &s.state && s.state != SlotState::Empty {
					run += 1;
				} else {
					run = 1;
				}
				
				last_symbol = &s.state;
				
				if run == 4 {
					return true;
				}
			}
			
			// Check possible diagonals
			// Get bottom of diagonal and then attempt to find a run by going to the top of the diagonal
			run = 1;
			if self.last_insert.0 + self.last_insert.1 <= 5 {
				// Look at diagonals above the mid point of the grid
				let mut bottom_of_diagonal = (0, self.last_insert.0 + self.last_insert.1);
				let top_of_diagonal = (bottom_of_diagonal.1, bottom_of_diagonal.0);
				last_symbol = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1].state;
				while bottom_of_diagonal != top_of_diagonal {
					bottom_of_diagonal = (bottom_of_diagonal.0 + 1, bottom_of_diagonal.1 - 1);
					let s = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1];
					
					if last_symbol == &s.state && s.state != SlotState::Empty {
						run += 1;
					} else {
						run = 1;
					}
					
					last_symbol = &s.state;
					
					if run == 4 {
						return true;
					}
				}
			} else {
				// Look at diagonals below the mid point of the grid
				let mut bottom_of_diagonal = (self.last_insert.0 + self.last_insert.1 - 5, 5);
				let top_of_diagonal = (6, bottom_of_diagonal.0 + bottom_of_diagonal.1 - 6);
				last_symbol = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1].state;
				while bottom_of_diagonal != top_of_diagonal {
					bottom_of_diagonal = (bottom_of_diagonal.0 + 1, bottom_of_diagonal.1 - 1);
					let s = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1];
					
					if last_symbol == &s.state && s.state != SlotState::Empty {
						run += 1;
					} else {
						run = 1;
					}
					
					last_symbol = &s.state;
					
					if run == 4 {
						return true;
					}
				}
			}
			
			// Now, do the same but go from the bottom of the antidiagonal to its top
			run = 1;
			if self.last_insert.0 <= self.last_insert.1 {
				let mut bottom_of_diagonal = (5 - self.last_insert.1 + self.last_insert.0, 5);
				let top_of_diagonal = (0, self.last_insert.1 - self.last_insert.0);
				last_symbol = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1].state;
				while bottom_of_diagonal != top_of_diagonal {
					bottom_of_diagonal = (bottom_of_diagonal.0 - 1, bottom_of_diagonal.1 - 1);
					let s = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1];
					
					if last_symbol == &s.state && s.state != SlotState::Empty {
						run += 1;
					} else {
						run = 1;
					}
					
					last_symbol = &s.state;
					
					if run == 4 {
						return true;
					}
				}
			} else {
				let mut bottom_of_diagonal = (6, 6 - self.last_insert.0 + self.last_insert.1);
				let top_of_diagonal = (self.last_insert.0 - self.last_insert.1, 0);
				last_symbol = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1].state;
				while bottom_of_diagonal != top_of_diagonal {
					bottom_of_diagonal = (bottom_of_diagonal.0 - 1, bottom_of_diagonal.1 - 1);
					let s = &self.columns[bottom_of_diagonal.0].slots[bottom_of_diagonal.1];
					
					if last_symbol == &s.state && s.state != SlotState::Empty {
						run += 1;
					} else {
						run = 1;
					}
					
					last_symbol = &s.state;
					
					if run == 4 {
						return true;
					}
				}
			}
			
			false
		}
	}
	
	#[allow(unused_must_use)]
	impl Display for Board {
		fn fmt(&self, f: &mut Formatter) -> Result {
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[0].display(), self.columns[1].slots[0].display(), self.columns[2].slots[0].display(), self.columns[3].slots[0].display(), self.columns[4].slots[0].display(), self.columns[5].slots[0].display(), self.columns[6].slots[0].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[1].display(), self.columns[1].slots[1].display(), self.columns[2].slots[1].display(), self.columns[3].slots[1].display(), self.columns[4].slots[1].display(), self.columns[5].slots[1].display(), self.columns[6].slots[1].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[2].display(), self.columns[1].slots[2].display(), self.columns[2].slots[2].display(), self.columns[3].slots[2].display(), self.columns[4].slots[2].display(), self.columns[5].slots[2].display(), self.columns[6].slots[2].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[3].display(), self.columns[1].slots[3].display(), self.columns[2].slots[3].display(), self.columns[3].slots[3].display(), self.columns[4].slots[3].display(), self.columns[5].slots[3].display(), self.columns[6].slots[3].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[4].display(), self.columns[1].slots[4].display(), self.columns[2].slots[4].display(), self.columns[3].slots[4].display(), self.columns[4].slots[4].display(), self.columns[5].slots[4].display(), self.columns[6].slots[4].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|{:^width$}|\n", self.columns[0].slots[5].display(), self.columns[1].slots[5].display(), self.columns[2].slots[5].display(), self.columns[3].slots[5].display(), self.columns[4].slots[5].display(), self.columns[5].slots[5].display(), self.columns[6].slots[5].display(), width=5);
			write!(f, "+-----------------------------------------+\n");
			write!(f, "{:^width$}{:^width$}{:^width$}{:^width$}{:^width$}{:^width$}{:^width$}\n", "1", "2", "3", "4", "5", "6", "7", width=6)
		}
	}
	
	struct Column {
		slots: [Slot; 6],
	}
	
	impl Column {
		fn new() -> Self {
			Column {
				slots: [Slot::new(), Slot::new(), Slot::new(), Slot::new(), Slot::new(), Slot::new()],
			}
		}
		
		fn contains_run_of_four(&self) -> bool {
			let mut run = 0;
			let mut last_symbol = &self.slots[5].state;
			self.slots.into_iter().rev()
				.any(|s| {
					if s.state == SlotState::Empty {
						return false;
					}
				
					if last_symbol == &s.state {
						run += 1;
					} else {
						run = 1;
					}
					
					last_symbol = &s.state;
					
					if run == 4 {
						true
					} else {
						false
					}
				})
		}
	}
	
	struct Slot {
		state: SlotState,
	}
	
	impl Slot {
		fn new() -> Self {
			Slot {
				state: SlotState::Empty,
			}
		}
		
		fn display(&self) -> &str {
			match self.state {
				SlotState::Empty => " ",
				SlotState::Red => "R",
				SlotState::Blue => "B",
			}
		}
	}
	
	#[derive(PartialEq)]
	pub enum SlotState {
		Empty,
		Red,
		Blue,
	}
}