use base64_url;
use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Caio Gondim <me@caiogondim.com>",
    version,
    about = "CLI encoder/decoder for base64url"
)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    decode: bool,

    input: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input.as_ref().unwrap();

    if cli.decode {
        let output = String::from_utf8(base64_url::decode(input).unwrap()).unwrap();
        println!("{}", output);
    } else {
        let output = base64_url::encode(input);
        println!("{}", output);
    }
}
