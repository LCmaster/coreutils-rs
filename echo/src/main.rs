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

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn test_cmd(args: &[&str], expected: &'static str) -> TestResult {
        Command::cargo_bin("echo")?
            .args(args)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn dies_no_arg() {
        let mut cmd = Command::cargo_bin("echo").unwrap();
        cmd.assert().failure().stderr(predicate::str::contains("Usage"));
    }

    #[test]
    fn print_hello_world() -> TestResult {
        test_cmd(&["hello,", "world!"], "hello, world!\n")
    }

    #[test]
    fn print_hello_world_no_newline() -> TestResult {
        test_cmd(&["-n", "hello,", "world!"], "hello, world!")
    }
}
