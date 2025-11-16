# 🦀 RustLingo

**RustLingo** is a powerful command-line tool written in Rust that translates text files from one language to another using the Google Translate API. Think of it as "Google Translate for files" — running locally on your machine!

## ✨ Features

- 📄 **Translate entire files** - Works with `.txt`, `.md`, `.json`, or any text-based file
- 🌍 **Support for 100+ languages** - Any language supported by Google Translate
- 🔍 **Auto-detection** - Automatically detects the source language if not specified
- 📁 **Smart output** - Creates output directories automatically if they don't exist
- 🎨 **Beautiful CLI** - Clean and intuitive command-line interface with verbose mode
- ⚡ **Fast and reliable** - Built with Rust for performance and safety

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or higher)
- A Google Cloud API key with Translation API enabled

### Building from source

```bash
# Clone the repository
git clone https://github.com/iamkabelomoobi/rustlingo.git
cd rustlingo

# Build the project
cargo build --release

# The binary will be in target/release/rustlingo
# Optionally, install it globally
cargo install --path .
```

## 🔑 Getting a Google Translate API Key

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the **Cloud Translation API**:
   - Navigate to "APIs & Services" > "Library"
   - Search for "Cloud Translation API"
   - Click "Enable"
4. Create credentials:
   - Go to "APIs & Services" > "Credentials"
   - Click "Create Credentials" > "API Key"
   - Copy your API key
5. (Optional) Set up billing - Google provides a free tier with generous limits

### Setting up your API key

You can provide your API key in two ways:

**Option 1: Environment variable (recommended)**
```bash
export GOOGLE_TRANSLATE_API_KEY="your-api-key-here"
```

Add this to your `~/.bashrc`, `~/.zshrc`, or equivalent to make it permanent.

**Option 2: Command-line flag**
```bash
rustlingo --api-key "your-api-key-here" -i input.txt -o output.txt -l es
```

## 🚀 Usage

### Basic Command

```bash
rustlingo -i input.txt -o output.txt -l es
```

This translates `input.txt` to Spanish and saves it as `output.txt`.

### Detailed Options

```bash
rustlingo [OPTIONS] --input <FILE> --output <FILE> --output-language <LANG>
```

#### Required Arguments

- `-i, --input <FILE>` - Path to the input file to translate
- `-o, --output <FILE>` - Path where the translated file will be saved
- `-l, --output-language <LANG>` - Target language code (e.g., `es`, `fr`, `de`, `zu`)

#### Optional Arguments

- `-s, --source-language <LANG>` - Source language code (auto-detects if not specified)
- `--api-key <KEY>` - Google Cloud API key (or use `GOOGLE_TRANSLATE_API_KEY` env var)
- `-v, --verbose` - Show detailed progress and information
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Common Language Codes

| Language | Code | Language | Code |
|----------|------|----------|------|
| English | `en` | Spanish | `es` |
| French | `fr` | German | `de` |
| Italian | `it` | Portuguese | `pt` |
| Chinese | `zh` | Japanese | `ja` |
| Korean | `ko` | Russian | `ru` |
| Arabic | `ar` | Hindi | `hi` |
| Zulu | `zu` | Xhosa | `xh` |

For a complete list, see [Google's language codes](https://cloud.google.com/translate/docs/languages).

## 📖 Examples

### Example 1: Basic translation

Translate an English text file to Spanish:

```bash
rustlingo -i readme.txt -o readme_es.txt -l es
```

### Example 2: With verbose output

See detailed progress information:

```bash
rustlingo -i input.md -o output_fr.md -l fr --verbose
```

Output:
```
🦀 RustLingo - File Translation Tool
=====================================
📄 Input:  input.md
📝 Output: output_fr.md

📖 Reading input file...
   ✓ Read 1234 characters

🌐 Sending translation request to Google Translate API...
   Source language: auto-detect
   Target language: fr
   ✓ Detected source language: en
   ✓ Translation complete

💾 Writing output file...
   ✓ Saved to output_fr.md

✨ Translation successful!
   input.md → output_fr.md
```

### Example 3: Specify source language

If you know the source language, specify it for better accuracy:

```bash
rustlingo -i document.txt -o translated/document_de.txt -s en -l de
```

### Example 4: Nested output directories

RustLingo automatically creates directories if they don't exist:

```bash
rustlingo -i input.txt -o translations/spanish/output.txt -l es
```

### Example 5: Translating a Markdown file

```bash
rustlingo -i README.md -o docs/README_ja.md -l ja --verbose
```

## 🛠️ Development

### Project Architecture

RustLingo uses a clean, modular architecture:

- **`src/main.rs`** - Application entry point
- **`src/lib.rs`** - Library interface for testing
- **`src/cli.rs`** - Command-line argument parsing
- **`src/translator.rs`** - Translation engine and API interaction
- **`src/file_io.rs`** - File reading and writing
- **`src/types.rs`** - Data structures and type definitions

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed documentation.

### Running tests

```bash
cargo test
```

### Building for development

```bash
cargo build
./target/debug/rustlingo --help
```

### Formatting and linting

```bash
cargo fmt
cargo clippy
```

## 📝 Example Workflow

**Input file (`hello.txt`):**
```text
Hello, how are you?
This is a Rust translation test.
Welcome to RustLingo!
```

**Command:**
```bash
rustlingo -i hello.txt -o hello_es.txt -l es --verbose
```

**Output file (`hello_es.txt`):**
```text
Hola, ¿cómo estás?
Esta es una prueba de traducción de Rust.
¡Bienvenido a RustLingo!
```

## ⚠️ Error Handling

RustLingo provides clear error messages:

- **File not found**: If the input file doesn't exist
- **API errors**: If there's an issue with the Google Translate API (invalid key, network issues, etc.)
- **Permission errors**: If the tool can't write to the output location
- **Invalid language codes**: If you specify an unsupported language

## 🔒 Privacy & Security

- Your API key is never logged or displayed in verbose mode
- Files are processed locally; only the text content is sent to Google's API for translation
- No data is stored or cached by RustLingo

## 📊 API Limits

Google Cloud Translation API has usage limits:

- **Free tier**: 500,000 characters per month
- After free tier: $20 per million characters
- Check your usage in the [Google Cloud Console](https://console.cloud.google.com/)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Uses [Google Cloud Translation API](https://cloud.google.com/translate)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- HTTP requests via [reqwest](https://github.com/seanmonstar/reqwest)

## 🐛 Issues & Support

If you encounter any issues or have questions:
- Open an issue on GitHub
- Check existing issues for solutions
- Include verbose output (`--verbose`) when reporting bugs

---

Made with ❤️ and Rust
