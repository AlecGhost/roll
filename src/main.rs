use clap::Parser;
use comfy_table::Table;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    IResult,
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

#[derive(Debug, PartialEq)]
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
    
    Ok((input, DiceRequest {
        count: count.unwrap_or(1),
        sides,
    }))
}

fn main() {
    let args = Args::parse();
    let mut table = Table::new();
    table.set_header(vec!["Die", "Roll"]);
    
    let mut total_sum: u64 = 0;
    
    for dice_str in &args.dice {
        match parse_dice_expression(dice_str) {
            Ok((remainder, request)) => {
                if !remainder.is_empty() {
                    eprintln!("Error: Invalid dice format '{}'. Unparsed content: '{}'", dice_str, remainder);
                    process::exit(1);
                }
                
                let mut rng = rand::thread_rng();
                for _ in 0..request.count {
                    let roll = rng.gen_range(1..=request.sides);
                    total_sum += roll as u64;
                    table.add_row(vec![
                        format!("d{}", request.sides),
                        format!("{}", roll),
                    ]);
                }
            },
            Err(_) => {
                eprintln!("Error: Failed to parse dice expression '{}'. Expected format 'NdS' (e.g. 1d20, 4d8).", dice_str);
                process::exit(1);
            }
        }
    }
    
    table.add_row(vec!["Total", &total_sum.to_string()]);
    
    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_simple() {
        let (_, res) = parse_dice_expression("1d20").unwrap();
        assert_eq!(res, DiceRequest { count: 1, sides: 20 });
    }

    #[test]
    fn test_parse_dice_implicit_count() {
        let (_, res) = parse_dice_expression("d6").unwrap();
        assert_eq!(res, DiceRequest { count: 1, sides: 6 });
    }

    #[test]
    fn test_parse_dice_multiple() {
        let (_, res) = parse_dice_expression("10d100").unwrap();
        assert_eq!(res, DiceRequest { count: 10, sides: 100 });
    }

    #[test]
    fn test_parse_dice_invalid() {
        assert!(parse_dice_expression("invalid").is_err());
        // valid prefix but remaining content check is done in main, parser returns residual
        let (rem, _) = parse_dice_expression("1d20extra").unwrap();
        assert_eq!(rem, "extra");
    }
}
