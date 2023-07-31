use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let message = "Hello, fellow Rustaceans!";
    let mut writer = BufWriter::new(stdout());

    say(message, message.len(), &mut writer).unwrap();
}
