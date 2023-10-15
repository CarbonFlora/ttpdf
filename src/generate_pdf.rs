use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use genpdf::{elements::Paragraph, Document, SimplePageDecorator};

use crate::{arguments::InitArgs, font::liberation_serif};

pub fn pdf() -> Result<()> {
    let arguments = InitArgs::parse();
    let mut text = read_to_string(arguments.text_file)?;
    text = header(&arguments.add_header, &text);

    let mut mod_name = arguments.name.clone();
    let font_family = liberation_serif()?;
    let mut doc = Document::new(font_family);
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(arguments.margins);

    doc.set_title(&mod_name);
    doc.set_page_decorator(decorator);
    doc.set_minimal_conformance();
    doc.set_line_spacing(arguments.spacing);

    let split_text = text.split('\n').collect::<Vec<&str>>();
    for line in split_text {
        doc.push(Paragraph::new(line));
    }

    if !arguments.name.contains('.') {
        mod_name += ".pdf";
    }
    doc.render_to_file(mod_name)?;

    Ok(())
}

pub fn header(add_header: &String, text: &str) -> String {
    let date = chrono::Local::now().to_string();
    let mut binding = String::from("Zi Hao Liang");
    binding += "\n";
    binding += &date;
    binding += "\n";
    binding += add_header;
    binding += "\n\n";
    binding += text;
    binding
}
