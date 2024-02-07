use anyhow::Result;
use regex::Regex;
use std::io::{BufRead, Write};

pub fn write_matches(re: Regex, reader: impl BufRead, mut writer: impl Write) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        let captures = re.captures(&line);
        captures
            .and_then(|c| c.get(1))
            .map(|c| writeln!(writer, "{}", c.as_str()));
    }
    Ok(())
}
