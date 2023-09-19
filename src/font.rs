use anyhow::Result;
use genpdf::fonts::{FontData, FontFamily};

pub fn liberation_serif() -> Result<FontFamily<FontData>> {
    Ok(FontFamily {
        regular: FontData::new(
            include_bytes!("fonts/liberationSerif-Regular.ttf").to_vec(),
            None,
        )?,
        bold: FontData::new(
            include_bytes!("fonts/liberationSerif-Bold.ttf").to_vec(),
            None,
        )?,
        italic: FontData::new(
            include_bytes!("fonts/liberationSerif-Italic.ttf").to_vec(),
            None,
        )?,
        bold_italic: FontData::new(
            include_bytes!("fonts/liberationSerif-BoldItalic.ttf").to_vec(),
            None,
        )?,
    })
}
