use structopt::StructOpt;
use jwt::{Header, Token, Unverified};
use std::{collections::BTreeMap};
use serde_json;

#[derive(StructOpt, Debug)]
#[structopt(name = "jwt-debugger")]
struct Opt {
    #[structopt(short, long)]
    raw: String
}

fn main() {
    let opt = Opt::from_args();
    let token: Token<Header, BTreeMap<String, serde_json::Value>, Unverified>  = Token::parse_unverified(&opt.raw).unwrap();
    println!("{}", serde_json::to_string_pretty(token.header()).unwrap());
    println!("{}", serde_json::to_string_pretty(token.claims()).unwrap());
}
