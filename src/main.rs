use clap::Parser;
use gen_pass::gen_pass;

#[derive(Parser, Debug)]
struct Args {
     #[arg(default_value_t = 8)]
    len: u16,
     #[arg(short, long, default_value_t = String::from("ulnp"))]
    chars: String
}
fn main() {
    let args = Args::parse();
    println!("{}", gen_pass(args.chars, args.len));
}
