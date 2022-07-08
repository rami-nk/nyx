use nyx::NyxCli;
use clap::Parser;

fn main() {
    let cli  = NyxCli::parse();
    
    nyx::run(cli);
}