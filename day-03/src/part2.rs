use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, nums) =
        delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(input)?;

    Ok((input, Instruction::Mul(nums.0, nums.1)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _do) = tag("do()")(input)?;

    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _do) = tag("don't()")(input)?;

    Ok((input, Instruction::Dont))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut result = Vec::new();
    let mut current = input;

    while !current.is_empty() {
        match parse_instruction(current) {
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
        .fold((0, true), |(sum, go), ins| match ins {
            Instruction::Mul(a, b) if go => (sum + a * b, go),
            Instruction::Do => (sum, true),
            Instruction::Dont => (sum, false),
            _ => (sum, go),
        })
        .0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
