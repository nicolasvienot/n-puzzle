use crate::board::utils::*;
use crate::algo::heuristics::{heuristic};
use crate::args::parser::{Config};
use crate::algo::search::{SearchType};

#[derive(Debug, Clone, PartialEq)]
pub enum Dir {
	N, E, S, W, None
}

impl Dir {
	pub fn value(&self) -> (i16, i16) {
		match *self {
			Dir::N => (0, -1),
			Dir::E => (1, 0),
			Dir::S => (0, 1),
			Dir::W => (-1, 0),
			Dir::None => (0, 0)
		}
	}
	pub fn is_horizontal(&self) -> bool {
		return *self == Dir::W || *self == Dir::E;
	}
	pub fn is_vertical(&self) -> bool {
		return !self.is_horizontal();
	}
}

pub fn new_position(position: (u16, u16), dir: (i16, i16)) -> (u16, u16) {
	return ((position.0 as i16 + dir.0) as u16, (position.1 as i16 + dir.1) as u16);
}

pub fn apply_action(size: u16, state: &Vec<u16>, current_pos: (u16, u16), new_pos: (u16, u16)) -> Result<Vec<u16>, ()> {
	let mut new_state: Vec<u16> = state.clone();
	if (0..(size)).contains(&(new_pos.0)) && (0..(size)).contains(&(new_pos.1)) {
		let index_a: u16 = fdtos(current_pos.0, current_pos.1, size);
		let index_b: u16 = fdtos(new_pos.0, new_pos.1, size);
		new_state.swap(index_a as usize, index_b as usize);
		return Ok(new_state);
	}
	return Err(());
}

fn get_neighbors(size: u16, state: &Vec<u16>) -> Vec<(Dir, Vec<u16>)> {
	let sd_pos: u16 = slot_pos(size, &state);
	let dd_pos: (u16, u16) = fstod(sd_pos, size);
	let positions = [Dir::N, Dir::E, Dir::S, Dir::W];
	let mut neighbors: Vec<(Dir, Vec<u16>)> = Vec::new();
	for pos in positions.iter() {
		match apply_action(size, &state, dd_pos, new_position(dd_pos, pos.value())) {
			Ok(new_state) => neighbors.push((pos.clone(), new_state)),
			Err(()) => {}
		}
	}
	return neighbors;
}

fn graph_search(size: u16, path: &mut Vec<(Dir, Vec<u16>)>, target: &Vec<u16>, cost: u32, bound: u32, explored_nodes: &mut u32, max_path_len: &mut u16, config: &Config) -> (bool, u32) {
	*explored_nodes += 1;
	let node = path.last().expect("Error: The path is empty");
	let new_cost: u32 = match config.search_type {
		SearchType::Normal => cost + heuristic(&config.heuristic, size, &node.1, target),
		SearchType::Greedy => heuristic(&config.heuristic, size, &node.1, target),
		SearchType::Uniform => cost
	};
	if new_cost > bound {
		return (false, new_cost);
	}
	else if node.1 == *target {
		return (true, new_cost);
	}
	let mut min: u32 = std::u32::MAX;
	for neighbour in get_neighbors(size, &node.1).iter() {
		if !path.contains(neighbour) {
			path.push(neighbour.clone());
			if path.len() as u16 > *max_path_len {
				*max_path_len = path.len() as u16;
			}
			match graph_search(size, path, target, cost + 1, bound, explored_nodes, max_path_len, config) {
				(res, _) if res => return (true, min),
				(_, val) if val < min => min = val,
				(_, _) => {}
			}
			path.pop();
		}
	}
	return (false, min);
}

pub fn resolve_puzzle(size: u16, path: &mut Vec<(Dir, Vec<u16>)>, target: &Vec<u16>, explored_nodes: &mut u32, max_path_len: &mut u16, config: &Config) {
	let node = path.last().expect("Error: The path has not been initialized");
	let mut bound = match config.search_type {
		SearchType::Normal | SearchType::Greedy => heuristic(&config.heuristic, size, &node.1, target),
		SearchType::Uniform => 0
	};
	println!("bound: {}", bound);
	loop {
		match graph_search(size, path, target, 0, bound, explored_nodes, max_path_len, config) {
			res if res.0 => break,
			res => bound = res.1
		}
		println!("new bound: {}", bound);
	}
}