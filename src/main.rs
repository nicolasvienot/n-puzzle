use std::time::{Instant};

extern crate npuzzle;
use npuzzle::board::create::{snail_generate};
use npuzzle::board::check::{is_solvable};
use npuzzle::args::handle::{handle_args};
use npuzzle::args::parser::{Config};
use npuzzle::algo::graph::{resolve_puzzle, Dir};
use npuzzle::board::utils::{get_all_states, factorial};
use npuzzle::visual::render::{start_visual};

fn main() {
	let config = Config::new();
	let (size, state) = handle_args(&config);

	println!("First state: {:?}", state);
	println!("Size: {}", size);
	if config.file.is_empty() {
		println!("Iterations: {}", config.iterations);
	}
	println!("Heuristic: {:?}", config.heuristic);
	println!("Search type: {}", config.search_type);

	let solvable: bool = is_solvable(size, state.clone());
	println!("Solvable: {:?}", solvable);

	if !solvable {
		panic!("Error: The puzzle is not solvable")
	}

	let target = snail_generate(size);
	println!("Target: {:?}", target);


	let mut path: Vec<(Dir, Vec<u16>)> = Vec::new();
	path.push((Dir::None, state.clone()));

	println!("-------");
	
	let start_time = Instant::now();
	let mut explored_nodes: u32 = 0;
	let mut max_path_len: u16 = 0;

	resolve_puzzle(size, &mut path, &target, &mut explored_nodes, &mut max_path_len, &config);
	
	println!("-------");
	
	let mut sequence = Vec::new();
	for node in path.iter() {
		if node.0 != Dir::None { sequence.push(node.0.clone()) }
	}

	println!("Solution: {:?}", sequence);
	println!("Number of moves: {:?}", path.len() - 1);
	println!("Complexity in time: {}", explored_nodes);
	println!("Complexity in size: {}", max_path_len);
	let max_states: u128 = factorial((size * size) as u128) / 2;
	if max_states > 0 {
		println!("Possible nb of solvable states: {:?}", max_states);
	} else {
		println!("Possible nb of solvable states is beyond imagination... (> 2^128)");
	}
	println!("Duration: {:?}s ({:?})", start_time.elapsed().as_secs(), start_time.elapsed());
	println!("-------");

	if config.visual == true {
		let board_array = get_all_states(state.clone(), size, &sequence);
		start_visual(board_array, size, start_time.elapsed().as_secs().to_string(), config.heuristic.to_string(), explored_nodes, max_path_len);
	}
}