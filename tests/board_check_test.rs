#[cfg(test)]
mod tests {
    extern crate npuzzle;
    use npuzzle::board::check::{is_solvable};

    #[test]
    fn solvable_1() {
        let vec = vec![3, 2, 6, 1, 4, 9, 8, 7, 5];
        let is_solvable: bool = is_solvable(3, vec);
        assert_eq!(is_solvable, true);
    }

    #[test]
    fn solvable_2() {
        let vec = vec![1, 11, 14, 12, 10, 13, 4, 8, 2, 16, 3, 6, 9, 7, 15, 5];
        let is_solvable: bool = is_solvable(4, vec);
        assert_eq!(is_solvable, true);
    }

    #[test]
    fn solvable_3() {
        let vec = vec![20, 12, 23, 13, 9, 21, 4, 14, 5, 6, 16, 17, 11, 18, 20, 19, 24, 1, 10, 15, 8, 22, 2, 3, 7];
        let is_solvable: bool = is_solvable(5, vec);
        assert_eq!(is_solvable, true);
    }

    #[test]
    fn unsolvable_1() {
        let vec = vec![1, 8, 2, 9, 4, 3, 7, 6, 5];
        let is_solvable: bool = is_solvable(3, vec);
        assert_eq!(is_solvable, false);
    }

    #[test]
    fn unsolvable_2() {
        let vec = vec![3, 9, 1, 15, 14, 11, 4, 6, 13, 16, 10, 12, 2, 7, 8, 5];
        let is_solvable: bool = is_solvable(4, vec);
        assert_eq!(is_solvable, false);
    }

    #[test]
    fn unsolvable_3() {
        let vec = vec![3, 12, 21, 15, 11, 22, 1, 9, 8, 6, 20, 16, 14, 13, 4, 19, 23, 18, 20, 10, 7, 24, 17, 2, 5];
        let is_solvable: bool = is_solvable(5, vec);
        assert_eq!(is_solvable, false);
    }
}
