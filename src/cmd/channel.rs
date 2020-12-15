#[derive(Debug, Clap)]
pub struct Channel {
  /// Search word
  #[clap(short = "s", long = "search")]
  search: Option<String>,
}
