mod args;
mod cag;

use crate::args::CommandParse;
use crate::args::Commands;
use crate::cag::caganalyzer;
use clap::Parser;
use figlet_rs::FIGfont;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-29
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("CAGRepeat");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::CAGRepeat {
            filepath,
            outputfile,
        } => {
            let command = caganalyzer(filepath, outputfile).unwrap();
            println!("The command has been finished:{}", command);
        }
    }
}
