/// Send Message to traQ
#[derive(Debug, Clap)]
pub struct Message {
  /// Message
  input: String,

  /// traQ Channnel
  #[clap(short = "ch", long = "channel")]
  channel: String,
}
