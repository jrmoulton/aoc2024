use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
}

// Clean parser for a single mul instruction
fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, nums) = delimited(
        tag("mul("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(nums.0, nums.1)))
}

// Fast parser that finds all instructions
fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut result = Vec::new();
    let mut current = input;

    while !current.is_empty() {
        match parse_mul(current) {
            Ok((remaining, instruction)) => {
                result.push(instruction);
                current = remaining;
            }
            Err(_) => {
                current = &current[1..];
            }
        }
    }

    Ok(("", result))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) =
        parse(input).map_err(|e| miette!("Failed to parse instructions: {}", e))?;

    let result: u32 = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
