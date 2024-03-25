use clap::Parser;
/// A simple implementation of `echo` in rust
#[derive(Parser, Debug)]
#[command(name = "echor")]
#[command(about, long_about = "None")]
#[command(author = "Kevin Monari")]
#[command(version = "0.01")]

struct Echor {
    #[arg(required=true, help="Input text")]
    text: Vec<String>,
    #[arg(long, short, help="Do not print new line")]
    new_line: bool,
}

fn main() {
    let echo = Echor::parse();
    let text = echo.text;
    let omit_newline = echo.new_line;

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
