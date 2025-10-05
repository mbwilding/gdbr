use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = false)]
pub struct Cli {
    /// Replay using rr
    #[arg(long, default_value_t = false)]
    pub rr_replay: bool,

    /// Path to the executable
    pub executable: Option<String>,

    /// Forwarded to GDB
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub gdb_args: Vec<String>,
}

impl Default for Cli {
    fn default() -> Self {
        use clap::Parser as _;
        Self::parse()
    }
}
