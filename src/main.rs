use clap::Parser;
use nyx::cl_args::NyxCli;

fn main() {
    let cli = NyxCli::parse();

    nyx::run(cli).unwrap();
}
