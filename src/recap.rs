use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Parser)]
struct Cli {
    /// The regex pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    info!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let re = Regex::new(&args.pattern)
        .with_context(|| format!("could not compile regex pattern: {}", &args.pattern))?;
    log::info!("regex: {:?}", re);

    let f = File::open(&args.path)
        .with_context(|| format!("could not read file {}", &args.path.display()))?;

    let reader = BufReader::new(f);
    let stdout = io::stdout();
    {
        let mut handle = io::BufWriter::new(stdout.lock());
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(captures) = re.captures(&line) {
                    if let Some(capture) = captures.get(1) {
                        writeln!(handle, "{}", capture.as_str())?;
                    }
                }
            }
        }
    }
    Ok(())
}
