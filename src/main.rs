use clap::Parser;

#[derive(Debug, Parser)]
struct CliArgs {
    /// The text to jam
    text: String,
    /// The charactor to insert for jamming
    #[clap(short, long, default_value = "/")]
    delimiter: String,
}

fn generate_jammed_string(text: &str, delimiter: &str) -> String {
    let jammed = text
        .split("")
        .filter(|st| st.len() > 0)
        .collect::<Vec<&str>>()
        .join(&delimiter);
    String::from(jammed)
}

fn main() {
    let args = CliArgs::parse();

    let text = args.text;
    let delimiter = args.delimiter;

    let jammed = generate_jammed_string(&text, &delimiter);

    println!("{}", jammed);
}

#[cfg(test)]
mod tests {
    #[test]
    fn generate_jammed_string() {
        assert_eq!(super::generate_jammed_string("abcdef", "/"), "a/b/c/d/e/f");
    }
}
