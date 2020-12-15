mod channel;
mod message;

use clap::Clap;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "traQ-CLI", about = "traQのコマンドラインツール")]
struct Opt {
  /// Activate debug mode
  // short and long flags (-d, --debug) will be deduced from the field's name
  #[structopt(short = "d", long = "debug", default_value = "false")]
  debug: bool,

  /// Sets a custom config file. Could have been an Option<T> with no default too
  #[clap(short = "c", long = "config", default_value = "default.conf")]
  config: PathBuf,

  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
  /// Send Message to traQ
  #[clap(name = "message")]
  Message(message::Message),

  /// Send Message to traQ
  #[clap(name = "channel")]
  Channel(channel::Channel),
}
