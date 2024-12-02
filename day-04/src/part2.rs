struct Map<'a> {
    text: &'a str,
    line_len: usize,
}
impl std::fmt::Debug for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("text", &"...")
            .field("line_len", &self.line_len)
            .finish()
    }
}

impl<'a> Map<'a> {
    fn new(text: &'a str) -> Self {
        // Add 1 to include the newline character
        let line_len = text.find('\n').unwrap() + 1;
        Self { text, line_len }
    }

    fn is_valid(&self, idx: usize) -> bool {
        debug_assert!(self.text.as_bytes()[idx] == b'A');

        let row = idx / self.line_len;
        let col = idx % self.line_len;
        let text_len = self.text.len();

        let direction_pairs = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

        for (dir1, dir2) in direction_pairs.iter() {
            let get_new_idx = |dir: &(isize, isize)| {
                let new_row = row as isize + dir.0;
                let new_col = col as isize + dir.1;

                if new_row >= 0 && new_col >= 0 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if new_col < self.line_len - 1 {
                        let new_idx = new_row * self.line_len + new_col;
                        if new_idx < text_len {
                            return Some(new_idx);
                        }
                    }
                }
                None
            };

            let idx1 = get_new_idx(dir1);
            let idx2 = get_new_idx(dir2);

            if let (Some(idx1), Some(idx2)) = (idx1, idx2) {
                if !self.pair_is_valid(&(idx1, idx2)) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    // requires that idx and new_idx are valid indicies
    fn pair_is_valid(&self, pair: &(usize, usize)) -> bool {
        debug_assert!((0..self.text.len()).contains(&pair.0));
        debug_assert!((0..self.text.len()).contains(&pair.1));

        let first = self.text.as_bytes()[pair.0];
        let second = self.text.as_bytes()[pair.1];
        matches!((first, second), (b'M', b'S') | (b'S', b'M'))
    }

    fn count(&self) -> u32 {
        self.text
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_idx, ch)| **ch == b'A')
            .filter(|(idx, _ch)| self.is_valid(*idx))
            .count() as u32
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let map = Map::new(input);
    Ok(map.count().to_string())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
