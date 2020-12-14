use crate::board::create::{snail_generate};

fn get_inversion_count(board: Vec<u16>, s: u16) -> i32 {
	let mut inv_count:i32 = 0;
	for i in 0..(s * s - 1) {
		for j in(i + 1)..(s * s) {
			if board[i as usize] != s * s && board[j as usize] != s * s && board[i as usize] > board[j as usize] {
				inv_count += 1;
			}
		}
	}
  return inv_count;
}

fn get_hole_x_from_bottom(board: Vec<u16>, s: u16) -> u16 {
	for i in 0..(s * s) {
		if board[i as usize] >= s * s {
			return s - i / s;
		}
	}
	return 0;
}

pub fn is_solvable(size: u16, board: Vec<u16>) -> bool {
	let solved_board: Vec<u16> = snail_generate(size);
	let inv_board: i32 = get_inversion_count(board.clone(), size);
	let inv_solved: i32 = get_inversion_count(solved_board.clone(), size);
	
	if size % 2 != 0 {
		return inv_board % 2 == inv_solved % 2
	} else {
		let mut x_pos: u16 = get_hole_x_from_bottom(board.clone(), size);
		if size % 4 == 2 {
			x_pos -= 1;
		}
		if x_pos % 2 == 1 {
			return inv_board % 2 != inv_solved % 2;
		} else {
			return inv_board % 2 == inv_solved % 2;
		}
	}
}