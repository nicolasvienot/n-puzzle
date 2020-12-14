use crate::algo::graph::{apply_action, new_position, Dir};

pub fn factorial(num: u128) -> u128 {
	match num {
		0 | 1 => 1,
		_ => match factorial(num - 1).checked_mul(num) {
            Some(x) => x,
            None => return 0,
        },
	}
}

pub fn fstod(index: u16, width: u16) -> (u16, u16) {
	return (index % width, index / width);
}

pub fn fdtos(x: u16, y: u16, width: u16) -> u16 {
	return y * width + x;
}

pub fn slot_pos(size: u16, state: &Vec<u16>) -> u16 {
	return state.iter().position(|&x| x == size * size).expect("Error: No slot found in state") as u16;
}

pub fn get_all_states(state: Vec<u16>, size: u16, sequence: &Vec<Dir>) -> Vec<Vec<u16>> {
	let mut state_updated: Vec<u16> = state.clone();
	let mut board_array: Vec<Vec<u16>> = Vec::new();
	board_array.push(state.clone());
	for pos in sequence.iter() {
		let sd_pos: u16 = slot_pos(size, &state_updated);
		let dd_pos: (u16, u16) = fstod(sd_pos, size);
		match apply_action(size, &state_updated, dd_pos, new_position(dd_pos, pos.value())) {
			Ok(new_state) => {
				board_array.push(new_state.clone());
				state_updated = new_state.clone();
			},
			Err(()) => {}
		}
	}
	return board_array;
}