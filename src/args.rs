use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "CAGrepeat",
    version = "1.0",
    about = "CAG repeat pattern.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// CAG repeat pattern
    CAGRepeat {
        /// path to the sequence file
        filepath: String,
    },
}
