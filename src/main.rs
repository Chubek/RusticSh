use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "shell.pest"]
struct ShellParser;


fn main()  {
    ShellParser::parse(Rule::complete_command, "echo 222")
        .expect("22")
        .next().unwrap();
}
