use clap::Parser;
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::{Command, ExitCode},
};

mod lexer;
mod token;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file: PathBuf,

    #[arg(long, group = "option")]
    lex: bool,

    #[arg(long, group = "option")]
    parse: bool,

    #[arg(long, group = "option")]
    codegen: bool,

    #[arg(long = "S")]
    s: bool,
}

fn main() -> ExitCode {
    // Parse command line arguments
    let args = Cli::parse();

    // First preprocess the file
    let preprocessed_file_path = args.file.with_extension("i");

    let status = Command::new("gcc")
        .args(["-E", "-P"])
        .arg(&args.file)
        .arg("-o")
        .arg(&preprocessed_file_path)
        .status()
        .expect("Failed to execute gcc command");

    if !status.success() {
        eprintln!("Failed to preprocess file");
        return ExitCode::FAILURE;
    }

    // Lex the file if requested
    //

    if !(args.lex || args.parse || args.codegen) {
        eprintln!("No action requested");
        return ExitCode::FAILURE;
    }
    let preprocessed_file = BufReader::new(
        File::open(preprocessed_file_path).expect("Failed to open preprocessed file"),
    );
    let mut tokenizer = lexer::Tokenizer::new(preprocessed_file);
    let tokens = tokenizer.tokenize().expect("Failed to tokenize file");

    // Parse the file if requested
    //

    // Codegen the file if requested
    //

    // Assemble and link the file
    // Just create a new process

    println!("Tokens are: {:?}", tokens);

    // Normal process execution
    ExitCode::SUCCESS
}
