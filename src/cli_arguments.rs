use clap::Parser;

#[derive(Parser, Default)]
#[command(about)]
#[clap(disable_help_flag = true)]
pub struct CliArguments {
    /// Show hidden files
    #[arg(short, long)]
    pub all: bool,

    /// Format file names. Appendds a symbol at the end indicating the type of file. Slash ('/') is
    /// for directories and at sign ('@') is for symbolic links.
    #[arg(short = 'F', long)]
    pub format: bool,

    /// Show the files in a list.
    #[arg(short, long)]
    pub list: bool,

    /// Show the files in a list.
    #[arg(short, long)]
    pub human: bool,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Path to list.
    pub path: String,
}
