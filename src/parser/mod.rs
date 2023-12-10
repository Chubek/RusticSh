use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use crate::semantics::*;



#[derive(Parser)]
#[grammar = "shell.pest"]
struct ShellParser;

fn parse_shell_command(pair: Pair<Rule>) -> Option<Box<ShAbsyn>> {
        match pair.as_rule() {
            Rule::complete_command => {
                let list_prime = parse_shell_command
                    (pair.into_inner().next().unwrap())
                    .unwrap();
                let list_second = parse_shell_command(
                    pair.into_inner().next().unwrap());


                Some(Box::new(ShAbsyn::CompleteCommand(list_prime, list_second)))
            }
            _ => None,
        }
}



pub fn generate_shell_ast(cmd: &'static str)  {
    let shell_command = ShellParser::parse(Rule::complete_command, &cmd)
             .expect("There was a problem parsing the commands")
             .next()
             .unwrap();
    parse_shell_command(shell_command);
}

