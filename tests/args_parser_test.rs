#[cfg(test)]
mod tests {
    extern crate npuzzle;
    use npuzzle::args::parser::*;
    use npuzzle::algo::heuristics::{Heuristic};
    use npuzzle::algo::search::{SearchType};

    #[test]
    fn test_no_args() {
        assert_eq!(
            Config::new_from(["exename"].iter()).unwrap(),
            Config { file: "".to_string(), size: 3, iterations: 100, heuristic: Heuristic::LinearConflict, search_type: SearchType::Normal, solvable: true, visual: false }
        );
    }

    #[test]
    fn test_complete_name() {
        assert_eq!(
            Config::new_from(["exename", "--unsolvable"].iter()).unwrap(),
            Config { file: "".to_string(), size: 3, iterations: 100, heuristic: Heuristic::LinearConflict, search_type: SearchType::Normal, solvable: false, visual: false }
        );
    }

    #[test]
    fn test_short_name() {
        assert_eq!(
            Config::new_from(["exename", "-f", "Hello"].iter()).unwrap(),
            Config { file: "Hello".to_string(), size: 3, iterations: 100, heuristic: Heuristic::LinearConflict, search_type: SearchType::Normal, solvable: true, visual: false }
        );
    }
}
