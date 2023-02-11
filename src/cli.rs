use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to password list .txt file (passwords seperated by newlines)
    #[arg(short, long)]
    pub password_list_path: String,

    /// Path to encrypted .dmg file
    #[arg(short, long)]
    pub dmg_path: String,

    /// Number of threads to run
    #[arg(short, long, default_value_t = 4)]
    pub thread_count: u8,
}