use rand::seq::SliceRandom;
use super::utils::{slot_pos};

fn snail(w: u16, h: u16, x: u16, y: u16) -> u16 {
	return if y != 0 {
		w + snail(h - 1, w, y - 1, w - x - 1)
	} else {
		x + 1
	};
}

pub fn snail_generate(size: u16) -> Vec<u16> {
	let mut target: Vec<u16> = Vec::new();
	for y in 0..size {
		for x in 0..size {
			target.push(snail(size, size, x, y));
		}
	}
	return target;
}

pub fn board_generate(s: u16, iterations: i32, solvable: bool) -> (u16, Vec<u16>) {
	let mut board: Vec<u16> = snail_generate(s);
	let mut choices: Vec<i16> = Vec::new();
	let mut switch_index: &i16;
	let mut pos: i16;
	let mut i: i32 = 0;
	while i < iterations {
		choices.clear();
		pos = slot_pos(s, &board) as i16;
		if pos % (s as i16) > 0 {
			choices.push(pos - 1);
		}
		if pos % (s as i16) < (s as i16) - 1 {
			choices.push(pos + 1);
		}
		if pos / (s as i16) > 0 && pos - (s as i16) >= 0 {
			choices.push(pos - (s as i16));
		}
		if pos / (s as i16) < (s as i16) - 1 {
			choices.push(pos + (s as i16));
		}
		switch_index = choices.choose(&mut rand::thread_rng()).expect("Error: The vec should not be empty");
		board[pos as usize] = board[*switch_index as usize];
		board[*switch_index as usize] = s * s;
		i += 1
	}
	if !solvable {
		if board[0] == s * s || board[1] == s * s {
			let tmp: u16 = board[(s * s) as usize - 1];
			board[(s * s) as usize - 1] = board[(s * s) as usize - 2];
			board[(s * s) as usize - 2] = tmp;
		}
		else {
			let tmp: u16 = board[0];
			board[0] = board[1];
			board[1] = tmp;
		}
	}
	return (s, board);
}