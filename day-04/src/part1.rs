use nom::AsChar;
use tracing::debug;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Valid {
    Valid(usize),
    Final,
    Invalid,
}

impl Valid {
    fn is_final(&self) -> bool {
        matches!(self, Self::Final)
    }
}

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

    fn direction_to_str(direction: u8) -> &'static str {
        match direction {
            0 => "top left diagonal",
            1 => "up",
            2 => "top right diagonal",
            3 => "right",
            4 => "bottom right diagonal",
            5 => "down",
            6 => "bottom left diagonal",
            7 => "left",
            _ => "invalid direction",
        }
    }

    fn get_next_ch(&self, idx: usize, direction: Option<u8>) -> [Valid; 8] {
        let row = idx / self.line_len;
        let col = idx % self.line_len;
        let text_len = self.text.len();

        let mut result = [Valid::Invalid; 8];

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];

        for (result_idx, (delta_row, delta_col)) in directions.iter().enumerate() {
            if let Some(direction) = direction {
                if result_idx as u8 != direction {
                    debug!(
                        "not checking direction: {}",
                        Self::direction_to_str(direction)
                    );
                    continue;
                }
            }
            let new_row = row as isize + delta_row;
            let new_col = col as isize + delta_col;

            // Check if new position is within bounds
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Check if new position is within line length and text bounds
                if new_col < self.line_len - 1 {
                    // -1 to exclude newline
                    let new_idx = new_row * self.line_len + new_col;
                    if new_idx < text_len {
                        debug!(
                            "checking direction: {}",
                            Self::direction_to_str(result_idx as u8)
                        );
                        result[result_idx] = self.next_is_valid(idx, new_idx);
                    }
                }
            }
        }
        result
    }

    // requires that idx and new_idx are valid indicies
    fn next_is_valid(&self, idx: usize, new_idx: usize) -> Valid {
        debug_assert!((0..self.text.len()).contains(&idx));
        debug_assert!((0..self.text.len()).contains(&new_idx));

        let current = self.text.as_bytes()[idx];
        let next = self.text.as_bytes()[new_idx];
        match (current, next) {
            (b'X', b'M') => Valid::Valid(new_idx),
            (b'M', b'A') => Valid::Valid(new_idx),
            (b'A', b'S') => Valid::Final,
            _ => Valid::Invalid,
        }
    }

    fn num_valid_paths(&self, idx: usize, direction: Option<u8>) -> u32 {
        debug!("{idx}:{}", self.text.as_bytes()[idx].as_char());
        let chs = self.get_next_ch(idx, direction);
        let mut sum = 0;
        for (direction, valid) in chs.into_iter().enumerate() {
            if valid.is_final() {
                sum += 1;
            } else if let Valid::Valid(idx) = valid {
                sum += self.num_valid_paths(idx, Some(direction as u8));
            }
        }
        debug!("{idx}:{}->{sum}\n", self.text.as_bytes()[idx].as_char());
        sum
    }

    fn count(&self) -> u32 {
        let mut count = 0;
        for idx in 0..self.text.len() {
            if self.text.as_bytes()[idx] == b'X' {
                count += self.num_valid_paths(idx, None);
            }
        }
        count
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let map = Map::new(input);
    Ok(map.count().to_string())
}

#[cfg(test)]
mod tests {
    // use tracing::Level;

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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
