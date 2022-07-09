use clap::Parser;
use nyx::NyxCli;

fn main() {
    let cli = NyxCli::parse();

    nyx::run(cli).unwrap();
}
