use clap::{App, Arg};
use regex::Regex;
fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let quote = "Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books. What do we seek through millions of pages?";

    let o_pattern = args.value_of("pattern");
    let o_re = o_pattern.and_then(|p| Regex::new(p).ok());
    if let Some(re) = o_re {
        for line in quote.lines() {
            if re.find(line).is_some() {
                println!("{}", line)
            }
        }
    }
}
