use clap::Parser;
#[derive(Parser, Default, Debug)]
#[command(name = "echo")]
#[command(author = "Luiz Vicente <luizcv89@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Echo the TEXT(s) to standard ouput", long_about = None)]
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
