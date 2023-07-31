use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = "Hello fellow Rustaceans!";
    let width = message.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}
