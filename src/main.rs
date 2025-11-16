use anyhow::Result;
use rustlingo::{cli::Args, file_io, translator::Translator};

fn main() -> Result<()> {
    let args = Args::parse_args();

    args.print_header();

    args.print_verbose("📖 Reading input file...");
    let content = file_io::read_file(&args.input)?;
    args.print_verbose(&format!("   ✓ Read {} characters", content.len()));
    args.print_verbose("");

    let translator = Translator::new(args.api_key.clone());
    let result = translator.translate(
        &content,
        &args.output_language,
        args.source_language.as_deref(),
        args.verbose,
    )?;

    args.print_verbose("💾 Writing output file...");
    file_io::write_file(&args.output, &result.translated_text)?;
    args.print_verbose(&format!("   ✓ Saved to {}", args.output.display()));
    args.print_verbose("");

    println!("✨ Translation successful!");
    println!("   {} → {}", args.input.display(), args.output.display());

    Ok(())
}
