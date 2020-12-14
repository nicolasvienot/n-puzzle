use crate::args::parser::Config;
use crate::board::create::board_generate;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;

fn load_file(file: &String) -> (u16, Vec<u16>) {
    if !metadata(file).expect("Error: A problem occured with the file").is_file() {
        panic!("Error: The file is a lie");
    }
    let file = File::open(file).expect("Error: File not found");
    let lines: Vec<_> = BufReader::new(file).lines().collect();
    let mut size: u16 = 0;
    let mut values: Vec<u16> = Vec::new();

    for line in lines.into_iter() {
        let offset = line.as_ref().expect("Error: Bad file format")[..].find('#').unwrap_or(line.as_ref().expect("Error: Bad file format").len());
        let drained: String = line.expect("Error: Bad line format").drain(..offset).collect();
        let split: Vec<_> = drained.split_whitespace().map(|s| s.to_string()).collect();
        if size > 0 && split.len() != size as usize && split.len() != 0 {
            panic!("Error: Bad map format");
        }
        for el in split.iter() {
            match el.parse::<i32>() {
                Ok(value) => match size == 0  {
                    true => {
                        if split.len() == 1 && value >= 2 && value < 100 {
                            size = value as u16;
                        } else {
                            panic!("Error: Bad value '{}', please indicate a valid map size", value);
                        }
                    },
                    false => {
                        if value >= 0 && (value as u16) < size * size && !values.contains(&(value as u16)) {
                            values.push(value as u16)
                        } else {
                            panic!("Error: Bad value '{}', values are not usable", value);
                        }
                    },
                },
                Err(_) => panic!("Error: Bad character"),
            }
        }
    }
    let zero_pos: usize = values.iter().position(|el| *el == 0).expect("Error: There should be a slot pos in map");
    values[zero_pos] = size * size;
    return (size, values);
}

pub fn handle_args(config: &Config) -> (u16, Vec<u16>) {
    if config.file.is_empty() {
        return board_generate(config.size, config.iterations, config.solvable);
    }
    return load_file(&config.file);
}
