use anyhow::{Context, Result};
use clap::Parser;
use log::info;
use regex::Regex;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Parser)]
struct Cli {
    /// The regex pattern to look for
    pattern: String,
}

fn write_matches(re: Regex, reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        let captures = re.captures(&line);
        captures
            .and_then(|c| c.get(1))
            .map(|c| writeln!(writer, "{}", c.as_str()));
    }
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    info!("input pattern: {:?}", args.pattern);

    let re = Regex::new(&args.pattern)
        .with_context(|| format!("could not compile regex pattern: {}", &args.pattern))?;
    log::info!("regex: {:?}", re);

    let reader = BufReader::new(io::stdin().lock());
    let stdout = io::stdout();
    {
        let mut handle = io::BufWriter::new(stdout.lock());
        write_matches(re, reader, &mut handle)?;
    }
    Ok(())
}
