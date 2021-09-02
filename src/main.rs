use grep_cli::stdout;
use pad::PadStr;
use regex::Regex;
use std::{
    error::Error,
    io::{self, BufRead, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let delim = Regex::new(r#"\s+"#).expect("Failed to compile regex");
    // Should handle `//` and `#`
    let comment = Regex::new(r#"^//|#"#).expect("Failed to compile regex");

    let input = io::stdin();

    // push all values onto vec, find the widest value in each column as we go
    // write rows with padding using widest value from each column + 1

    let mut widths = vec![];
    let mut table: Vec<Vec<_>> = vec![];
    for line in input.lock().lines() {
        let line = line?;
        // TODO: make any comment char
        if comment.is_match(line.trim_start()) {
            table.push(vec![line.trim().to_owned()]);
        } else {
            let mut row = vec![];
            for (i, value) in delim.split(line.trim()).enumerate() {
                if let Some(width) = widths.get(i) {
                    if value.len() > *width {
                        widths[i] = value.len();
                    }
                } else {
                    widths.push(value.len());
                }
                row.push(value.to_owned());
            }
            table.push(row);
        }
    }

    let mut writer = stdout(cli_table::ColorChoice::Never);
    for row in table {
        for (i, value) in row.iter().enumerate() {
            write!(&mut writer, "{}", value.pad_to_width(widths[i] + 1))?;
        }
        writeln!(&mut writer)?;
    }

    Ok(())
}
