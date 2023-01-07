use clap::Parser;
#[derive(Parser, Default, Debug)]
#[command(name = "echo")]
#[command(author = "Luiz Vicente <luizcv89@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Echo the TEXT(s) to standard ouput", long_about = None)]
#[command(arg_required_else_help(true))]
struct Arguments {
    ///Do not output a trailing newline
    #[arg(short = 'n')]
    disable_newline:bool,
    
    texts: Vec<String>

}

fn main() {
    let args = Arguments::parse();
    let output = args.texts.join(" ");
    let ending = if args.disable_newline { "" } else { "\n" };

    print!("{}{}", output, ending);
}

#[cfg(test)]
mod tests {
    use assert_cmd::{Command, output};
    use predicates::prelude::*;

    #[test]
    fn dies_no_arg() {
        let mut cmd = Command::cargo_bin("echo").unwrap();
        cmd.assert().failure().stderr(predicate::str::contains("Usage"));
    }

    #[test]
    fn print_hello() {
        let mut cmd = Command::cargo_bin("echo").unwrap();
        cmd.arg("hello").assert().success().stdout("hello\n");
    }

    #[test]
    fn print_hello_no_trailing_newline() {
        let mut cmd = Command::cargo_bin("echo").unwrap();
        cmd.args(vec!["-n", "hello"]).assert().success().stdout("hello");
    }
}