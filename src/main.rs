use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    /// The text to jam
    text: String,
    /// The charactor to insert for jamming
    #[clap(short, long, default_value = "/")]
    delimiter: String,
}

fn main() {
    let args = CliArgs::parse();
    println!("Hello, world! {:#?}", args);
}
