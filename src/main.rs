#[derive(Debug)]
struct Args {
    input: String,
    output: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_args() -> Result<Args> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        input: args.value_from_str("--input")?,
        output: args.value_from_str("--output")?,
    };

    Ok(args)
}

fn main() -> Result<()> {
    let args = parse_args()?;

    Ok(())
}
