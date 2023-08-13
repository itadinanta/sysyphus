use clap::Parser;
use log4rs::append::console::ConsoleAppender;

pub mod clock;
mod app;
mod gather;

/// Simple program to greet a person
#[derive(Parser, Debug)] // Just to make testing across clap features easier
struct Args {
	/// Implicitly using `std::str::FromStr`
	#[arg(short = 'O')]
	optimization: Option<usize>,

	/// Allow invalid UTF-8 paths
	#[arg(short = 'I', value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
	include: Option<std::path::PathBuf>,

	/// Handle IP addresses
	#[arg(long)]
	bind: Option<std::net::IpAddr>,
}

fn main() {
	use log4rs::config::*;

	let log_appender = Appender::builder()
		.build("stdout".to_string(), Box::new(ConsoleAppender::builder().build()));

	let log_root = Root::builder().appender("stdout".to_string()).build(log::LevelFilter::Info);
	let log_config = Config::builder().appender(log_appender).build(log_root);

	init_config(log_config.unwrap()).unwrap();

	// let args = Args::parse();

	app::App::default().run();
}
