use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    mode: Option<Modes>,
}

#[derive(Subcommand, Debug)]
enum Modes {
    Server {
        #[arg(short,long,default_value_t=9000)]
        port: i32,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.mode {
        Some(Modes::Server { port }) => {
            println!("Server start on {:?}",port);
        }
        None => {}
    }
}
