use std::fmt;
use rulinalg::matrix::{Matrix, BaseMatrixMut, BaseMatrix, MatrixSlice};

#[derive(Debug, Clone, PartialEq)]
pub enum Heuristic {
   Manhattan,
   Euclidian,
   Hamming,
   LinearConflict
}

impl fmt::Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn heuristic(heuristic: &Heuristic, size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
   return match heuristic {
	  Heuristic::Manhattan => manhattan(size, state, target),
	  Heuristic::Euclidian => euclidian(size, state, target),
	  Heuristic::Hamming => hamming_distance(size, state, target),
	  Heuristic::LinearConflict => linear_conflict(size as usize, state, target)
   }
}

fn manhattan(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
    let mut dist: u32 = 0;
    for i in 0..(state.len()){
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size as i32 - destination_index as i32 % size as i32).abs();
            let y = (i as i32 / size as i32 - destination_index as i32 / size as i32).abs();
            dist += (x + y) as u32;
        }
    }
    return dist;
}

fn euclidian(size: u16, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
    let mut dist: f32 = 0.0;
    for i in 0..(state.len()) {
        if state[i] != size * size {
            let destination_index: usize = target.iter().position(|&x| x == state[i]).unwrap_or(0);
            let x = (i as i32 % size as i32 - destination_index as i32 % size as i32).pow(2);
            let y = (i as i32 / size as i32 - destination_index as i32 / size as i32).pow(2);
            dist += (x as f32 + y as f32).sqrt();
        }
    }
    return dist as u32;
}

fn hamming_distance(size: u16, state: &Vec<u16>, target: &[u16]) -> u32 {
    let mut misplaced: u32 = 0;
    for i in 0..(size * size) {
        if state[i as usize] != size * size {
           if state[i as usize] != target[i as usize] {
                misplaced += 1;
            }
        }
    }
    return misplaced;
}

fn find_conflicts(size: usize, line: MatrixSlice<u16>, target_line: MatrixSlice<u16>) -> Matrix<u16> {
    let mut conflicts_matrix: Matrix<u16> = Matrix::<u16>::zeros(size as usize, size as usize);
    for index_a in 0..size {
        let value_a = line.iter().nth(index_a).expect("Error: Overflow occured in find_conflicts");
        if *value_a != (size * size) as u16 {
            let target_a = target_line.iter().position(|x| x == value_a);
            if target_a.is_some() {
                for index_b in (index_a + 1)..size {
                    let value_b = line.iter().nth(index_b).expect("Error: Overflow occured in find_conflicts");
                    if *value_b != (size * size) as u16 {
                        let target_b = target_line.iter().position(|x| x == value_b);
                        if target_b.is_some() && target_b < target_a {
                            conflicts_matrix.row_mut(index_a)[index_b] = 1;
                            conflicts_matrix.row_mut(index_b)[index_a] = 1;
                        }
                    }
                }
            }
        }
    }
    return conflicts_matrix;
}

fn line_extra_moves(size: usize, line: MatrixSlice<u16>, target_line: MatrixSlice<u16>) -> u16 {
    let mut total_conflicting_tiles: u16 = 0;
    let mut conflicts_matrix: Matrix<u16> = find_conflicts(size, line, target_line);
    while conflicts_matrix.sum() > 0 {
        let conflicts_table = conflicts_matrix.sum_cols();
        let most_conflicting_tile: usize = conflicts_table.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index).expect("Error: most_conflicting_tile is empty");
        *conflicts_matrix.row_mut(most_conflicting_tile) *= 0;
        *conflicts_matrix.col_mut(most_conflicting_tile) *= 0;
        total_conflicting_tiles += 1;
    }
    return total_conflicting_tiles;
}

fn linear_conflict(size: usize, state: &Vec<u16>, target: &Vec<u16>) -> u32 {
    let state_matrix: Matrix<u16> = Matrix::<u16>::new(size as usize, size as usize, state.clone());
    let target_matrix: Matrix<u16> = Matrix::<u16>::new(size as usize, size as usize, target.clone());
    let mut extra_moves: u16 = 0;
    let mut row: rulinalg::matrix::Row<u16>;
    let mut target_row: rulinalg::matrix::Row<u16>;
    for row_index in 0..size {
        row = state_matrix.row(row_index);
        target_row = target_matrix.row(row_index);
        extra_moves += line_extra_moves(size, row.as_slice(), target_row.as_slice());
    }
    let mut col: rulinalg::matrix::Column<u16>;
    let mut target_col: rulinalg::matrix::Column<u16>;
    for col_index in 0..size {
        col = state_matrix.col(col_index);
        target_col = target_matrix.col(col_index);
        extra_moves += line_extra_moves(size, col.as_slice(), target_col.as_slice());
    }
    let md = manhattan(size as u16, state, target);
    return md + (2 * extra_moves) as u32;
}