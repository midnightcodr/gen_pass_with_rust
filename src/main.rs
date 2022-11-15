use clap::Parser;
use gen_pass::gen_pass;

#[derive(Parser, Debug)]
struct Args {
     #[arg(default_value_t = 8)]
    len: u8,
     #[arg(short, long, default_value_t = String::from("ulnp"))]
    chars: String
}
fn main() {
    let args = Args::parse();
    // let len: u8 = std::env::args().nth(1).expect("Please provide length").parse().expect("Please provide a positive integer");
    println!("{}", gen_pass(args.chars, args.len));
    // println!("len={}, chars={}", &args.len, args.chars);
}
