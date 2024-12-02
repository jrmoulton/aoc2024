use nom::{
    character::complete::{self, newline, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut left, mut right)) =
        parse(input).map_err(|e| miette::miette!("parse failed {}", e))?;

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum::<u32>();
    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, space1, complete::u32),
            opt(newline),
        ),
        || (Vec::new(), Vec::new()),
        |mut acc, (l, r)| {
            acc.0.push(l);
            acc.1.push(r);
            acc
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
