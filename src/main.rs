use jwt::{Header, Token, Unverified};
use std::collections::BTreeMap;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jwt-debugger")]
struct Opt {
    #[structopt(long, help = "A string representing a serialized JWT")]
    raw: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let token: Token<Header, BTreeMap<String, serde_json::Value>, Unverified> =
        Token::parse_unverified(&opt.raw)?;
    println!("{}", serde_json::to_string_pretty(token.header())?);
    println!("{}", serde_json::to_string_pretty(token.claims())?);
    Ok(())
}
