use pest_derive::Parser;
use pest::Parser;


#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()>{
    let got = Grammar::parse(Rule::file, "-123.34,-56\n")?;    
    println!("{:?}", got);

    Ok(())
}
