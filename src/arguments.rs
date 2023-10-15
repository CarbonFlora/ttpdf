use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ttpdf")]
#[command(author = "Zi Hao Liang <zihaoliang0413@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about="Text to PDF utility.", long_about=None)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = String::from("Untitled"))]
    pub name: String,

    #[arg(short, long, default_value_t = 10)]
    pub margins: u32,

    #[arg(short, long, default_value_t = 1.25)]
    pub spacing: f64,

    #[arg(short, long, default_value_t = String::from(""))]
    pub add_header: String,

    #[arg(required = true)]
    pub text_file: String,
}
