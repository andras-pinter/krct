use krct::Krct;

#[derive(structopt::StructOpt)]
#[structopt(name = "krct", about = "A simple toy payments engine.")]
struct Args {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

#[paw::main]
fn main(args: Args) {
    match Krct::read(args.input) {
        Ok(krct) => {
            if let Err(err) = krct.dump() {
                eprint!("{}", err)
            }
        }
        Err(err) => eprint!("{}", err),
    }
}
