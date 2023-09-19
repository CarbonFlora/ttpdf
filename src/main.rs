use anyhow::Result;
use ttpdf::generate_pdf::{generate_pdf, parse_inputs};

fn main() -> Result<()> {
    let input = parse_inputs()?;
    generate_pdf(&input.0, &input.1)?;
    Ok(())
}
