use anyhow::{Result, anyhow, bail};
use clap::Parser;
use comfy_table::Table;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
};
use rand::Rng;
use std::process;

/// A simple CLI to roll dice
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Dice expressions (e.g. 1d20, 4d8)
    #[arg(required = true)]
    dice: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct DiceRequest {
    count: u32,
    sides: u32,
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_dice_expression(input: &str) -> IResult<&str, DiceRequest> {
    let (input, count) = opt(parse_u32)(input)?;
    let (input, _) = tag("d")(input)?;
    let (input, sides) = parse_u32(input)?;

    Ok((
        input,
        DiceRequest {
            count: count.unwrap_or(1),
            sides,
        },
    ))
}

fn parse_and_validate(s: &str) -> Result<DiceRequest> {
    let (remainder, request) = parse_dice_expression(s).map_err(|_| {
        anyhow!(
            "Error: Failed to parse dice expression '{}'. Expected format 'NdS' (e.g. 1d20, 4d8).",
            s
        )
    })?;

    if !remainder.is_empty() {
        bail!(
            "Error: Invalid dice format '{}'. Unparsed content: '{}'",
            s,
            remainder
        );
    }

    if request.sides == 0 {
        bail!("Error: Dice cannot have 0 sides.");
    }

    Ok(request)
}

fn roll_dice(requests: &[DiceRequest]) -> Vec<(u32, u32)> {
    requests
        .iter()
        .flat_map(|req| {
            (0..req.count).map(move |_| {
                let mut rng = rand::thread_rng();
                (req.sides, rng.gen_range(1..=req.sides))
            })
        })
        .collect()
}

fn execute_roll(dice_args: &[String]) -> Result<String> {
    // 1. Parse and Validate Inputs
    let requests: Vec<DiceRequest> = dice_args
        .iter()
        .map(|s| parse_and_validate(s))
        .collect::<Result<_>>()?;

    // 2. Perform Calculations
    let results = roll_dice(&requests);

    // 3. Format Output
    let mut table = Table::new();
    table.set_header(vec!["Die", "Roll"]);

    let total_sum: u64 = results.iter().map(|(_, roll)| *roll as u64).sum();

    for (sides, roll) in results {
        table.add_row(vec![format!("d{}", sides), roll.to_string()]);
    }

    table.add_row(vec!["Total", &total_sum.to_string()]);

    Ok(table.to_string())
}

fn main() {
    let args = Args::parse();
    match execute_roll(&args.dice) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Parser Unit Tests ---

    #[test]
    fn test_parse_dice_simple() {
        let (_, res) = parse_dice_expression("1d20").unwrap();
        assert_eq!(
            res,
            DiceRequest {
                count: 1,
                sides: 20
            }
        );
    }

    #[test]
    fn test_parse_dice_implicit_count() {
        let (_, res) = parse_dice_expression("d6").unwrap();
        assert_eq!(res, DiceRequest { count: 1, sides: 6 });
    }

    #[test]
    fn test_parse_dice_multiple() {
        let (_, res) = parse_dice_expression("10d100").unwrap();
        assert_eq!(
            res,
            DiceRequest {
                count: 10,
                sides: 100
            }
        );
    }

    #[test]
    fn test_parse_dice_invalid() {
        assert!(parse_dice_expression("invalid").is_err());
        let (rem, _) = parse_dice_expression("1d20extra").unwrap();
        assert_eq!(rem, "extra");
    }

    // --- Integration Tests (using function calls) ---

    #[test]
    fn test_single_die() {
        let output = execute_roll(&["1d20".to_string()]).unwrap();
        assert!(output.contains("d20"));
        assert!(output.contains("Die")); // Header
        assert!(output.contains("Roll")); // Header
    }

    #[test]
    fn test_multiple_dice() {
        let output = execute_roll(&["2d6".to_string(), "1d10".to_string()]).unwrap();
        assert!(output.contains("d6"));
        assert!(output.contains("d10"));
        assert!(output.contains("Total"));
    }

    #[test]
    fn test_invalid_arg() {
        let err = execute_roll(&["invalid".to_string()]).unwrap_err();
        assert!(err.to_string().contains("Failed to parse dice expression"));
    }

    #[test]
    fn test_partial_valid_arg() {
        let err = execute_roll(&["1d20extra".to_string()]).unwrap_err();
        assert!(err.to_string().contains("Invalid dice format"));
    }

    #[test]
    fn test_zero_sides() {
        let err = execute_roll(&["2d0".to_string()]).unwrap_err();
        assert!(err.to_string().contains("Dice cannot have 0 sides"));
    }
}
