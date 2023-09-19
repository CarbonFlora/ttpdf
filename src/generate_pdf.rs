use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use genpdf::{elements::Paragraph, Document, SimplePageDecorator};

use crate::{arguments::InitArgs, font::liberation_serif};

pub fn parse_inputs() -> Result<(String, String)> {
    let arguments = InitArgs::parse();
    let text = read_to_string(arguments.text_file)?;
    Ok((header(&text), arguments.name))
}

pub fn generate_pdf(text: &str, name: &str) -> Result<()> {
    let mut mod_name = name.to_string();
    let font_family = liberation_serif()?;
    let mut doc = Document::new(font_family);
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);

    doc.set_title(&mod_name);
    doc.set_page_decorator(decorator);
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let split_text = text.split('\n').collect::<Vec<&str>>();
    for line in split_text {
        doc.push(Paragraph::new(line));
    }

    if !name.contains('.') {
        mod_name += ".pdf";
    }
    doc.render_to_file(mod_name)?;

    Ok(())
}

pub fn header(text: &str) -> String {
    let date = chrono::Local::now().to_string();
    let mut binding = String::from("Zi Hao Liang");
    binding += "\n";
    binding += &date;
    binding += "\n\n";
    binding += text;
    binding
}
