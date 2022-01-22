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

    let text = args.text;
    let delimiter = args.delimiter;

    let jammed = text
        .split("")
        .filter(|st| st.len() > 0)
        .collect::<Vec<&str>>()
        .join(&delimiter);

    println!("{}", jammed);
}
