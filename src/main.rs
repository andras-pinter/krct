use krct::Krct;

#[derive(structopt::StructOpt)]
#[structopt(name = "krct", about = "A simple toy payments engine.")]
struct Args {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

/// Main entry point. Requires an input CSV file and the result is dumped to stdout.
///
/// # Example
/// ```bash
/// $ cargo run --release -- input.csv > output.csv
/// ```
#[paw::main]
fn main(args: Args) {
    match Krct::read(args.input) {
        Ok(krct) => {
            if let Err(err) = krct.dump(std::io::stdout()) {
                eprint!("{}", err)
            }
        }
        Err(err) => eprint!("{}", err),
    }
}
