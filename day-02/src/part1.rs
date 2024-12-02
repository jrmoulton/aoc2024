use nom::{
    character::complete::{self, *},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut all_increasing = true;
        let mut all_decreasing = true;

        for pair in self.levels.windows(2) {
            let (Some(v1), Some(v2)) = (pair.first(), pair.get(1)) else {
                unreachable!();
            };
            if v1 == v2 {
                return false;
            }
            if v1.abs_diff(*v2) > 3 {
                return false;
            }
            if v1 < v2 {
                all_decreasing = false;
            }
            if v1 > v2 {
                all_increasing = false;
            }
            if !all_decreasing && !all_increasing {
                return false;
            }
        }
        true
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette::miette!("Failed to parse input: {}", e))?;

    let res = reports.iter().filter(|r| r.is_safe()).count();

    Ok(res.to_string())
}

// Parse a single line into a Report struct
fn parse_line(input: &str) -> IResult<&str, Report> {
    let (input, levels) = terminated(separated_list1(space1, complete::u32), opt(newline))(input)?;
    Ok((input, Report { levels }))
}

// Parse all lines into Vec<Report>
fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    many1(parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
