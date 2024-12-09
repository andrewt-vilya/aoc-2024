use crate::util::{cartesian_product, Vec2};

const TARGET_XMAS: [u8;4] = *b"XMAS";
const TARGET_MAS: [u8;3] = *b"MAS";

const INPUT: &str = include_str!("../inputs/input-4.txt");

struct Grid<'a> {
    data: &'a [u8],
    size: Vec2,
}

impl<'a> Grid<'a> {
    fn from_ascii(input: &'a str) -> Self {
        assert!(input.is_ascii(), "Non-ASCII input");

        let width = input.lines().next().unwrap_or_default().len();
        assert!(input.lines().all(|line| line.len() == width), "Inconsistent line length");

        Self {
            size: Vec2::new(width, input.lines().count()),
            data: input.as_bytes(),
        }
    }

    fn get(&self, Vec2 { x, y }: Vec2) -> Option<u8> {
        self.data.get((y * (self.size.x + "\n".len())) + x).copied()
    }

    fn get_signed(&self, coord: Vec2<isize>) -> Option<u8> {
        self.get(coord.convert()?)
    }

    pub fn all_positions(&self) -> impl Iterator<Item=Vec2> + Clone {
        cartesian_product(0..self.size.y, 0..self.size.x).map(Vec2::from)
    }

    fn is_xmas(&self, position: Vec2, direction: Vec2<isize>) -> bool {
        let position = position.convert::<isize>().expect("Grid too large for platform pointer");

        TARGET_XMAS.iter().copied().enumerate().all(|(i, item)|
            self.get_signed(position + direction * i as isize) == Some(item)
        )
    }

    fn count_xmas(&self) -> usize {
        cartesian_product(self.all_positions(), Vec2::all_directions())
            .filter(|&(position, direction)| self.is_xmas(position, direction))
            .count()
    }

    fn is_mas_x(&self, position: Vec2) -> bool {
        fn matches(predicate: impl Fn((usize, u8)) -> bool) -> bool {
            TARGET_MAS   .iter().copied()      .enumerate().all(&predicate)
            || TARGET_MAS.iter().copied().rev().enumerate().all(&predicate)
        }

        let position = position.convert::<isize>().expect("Grid too large for platform pointer");

        if !matches(|(i, letter)| {
            let diagonal_down = position + Vec2::new(i as isize - 1, i as isize - 1);
            self.get_signed(diagonal_down) == Some(letter)
        }) {
            return false;
        }

        if !matches(|(i, letter)| {
            let diagonal_up = position + Vec2::new(i as isize - 1, 1 - i as isize);
            self.get_signed(diagonal_up) == Some(letter)
        }) {
            return false;
        }

        true
    }

    fn count_mas_xes(&self) -> usize {
        self.all_positions()
            .filter(|&position| self.is_mas_x(position))
            .count()
    }
}

pub fn main() {
    let grid = Grid::from_ascii(INPUT);

    println!("Part 1 | Count: {}", grid.count_xmas());
    println!("Part 2 | Count: {}", grid.count_mas_xes());
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const PARTIAL_INPUT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    " };

    #[test]
    fn test_grid() {
        let input = indoc! {"
            123
            456
            789
        "};
        let grid = Grid::from_ascii(input);

        assert_eq!(grid.size, Vec2::new(3, 3));
        assert_eq!(grid.get(Vec2::new(0, 0)), Some(b'1'));
        assert_eq!(grid.get(Vec2::new(0, 1)), Some(b'4'));
        assert_eq!(grid.get(Vec2::new(2, 2)), Some(b'9'));
        assert_eq!(grid.get(Vec2::new(3, 3)), None);
    }

    #[test]
    fn test_xmas_partial() {
        let grid = Grid::from_ascii(PARTIAL_INPUT);

        assert_eq!(Vec2::new(10, 10), grid.size);
        assert_eq!(18, grid.count_xmas());
    }

    #[test]
    fn test_mas_x_partial() {
        let grid = Grid::from_ascii(PARTIAL_INPUT);

        assert_eq!(Vec2::new(10, 10), grid.size);
        assert_eq!(9, grid.count_mas_xes());
    }
}
