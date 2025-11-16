use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: PathBuf,

    #[arg(short = 'l', long)]
    pub output_language: String,

    #[arg(short = 's', long)]
    pub source_language: Option<String>,

    #[arg(long, env = "GOOGLE_TRANSLATE_API_KEY")]
    pub api_key: String,

    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn print_header(&self) {
        if self.verbose {
            println!("🦀 RustLingo - File Translation Tool");
            println!("=====================================");
            println!("📄 Input:  {}", self.input.display());
            println!("📝 Output: {}", self.output.display());
            println!();
        }
    }

    pub fn print_verbose(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }
}
