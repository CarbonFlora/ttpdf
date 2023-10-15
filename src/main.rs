use anyhow::Result;
use ttpdf::generate_pdf::pdf;

fn main() -> Result<()> {
    pdf()?;
    Ok(())
}
