use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("numbers.csv").expect("cannot read file");
    let file = CSVParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    print!("{:?}",file);
}