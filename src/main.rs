use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate serde;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Args {
    input: String,
    output: String,
}

#[derive(Debug, Deserialize)]
struct Notebook {
    cells: Vec<NotebookCell>,
}

#[derive(Debug, Deserialize)]
struct NotebookCell {
    cell_type: String,
    source: Vec<String>,
}

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells.iter().filter(|c| c.cell_type == "code") {
            for line in &cell.source {
                write!(f, "{}", line)?;
            }

            write!(f, "\n\n")?;
        }

        Ok(())
    }
}

fn parse_args() -> Result<Args> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        input: args.value_from_str(["-i", "--input"])?,
        output: args.value_from_str(["-o", "--output"])?,
    };

    Ok(args)
}

fn main() -> Result<()> {
    let args = parse_args()?;

    // Read the input file and decode it
    let input = File::open(&args.input)?;
    let reader = BufReader::new(input);
    let notebook: Notebook = serde_json::from_reader(reader)?;

    let formatted = notebook.to_string();

    std::fs::write(&args.output, &formatted)?;

    Ok(())
}
