#![allow(unused)]

use clap::Parser;
use log4rs::append::console::ConsoleAppender;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
	menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
	TrayIconBuilder, TrayIconEvent,
};

mod app;
pub mod clock;
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

fn event_main() {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");
	let icon = load_icon(std::path::Path::new(path));

	let event_loop = EventLoopBuilder::new().build();

	let tray_menu = Menu::new();

	let quit_i = MenuItem::new("Quit", true, None);
	tray_menu.append_items(&[
		&PredefinedMenuItem::about(
			None,
			Some(AboutMetadata {
				name: Some("tao".to_string()),
				copyright: Some("Copyright tao".to_string()),
				..Default::default()
			}),
		),
		&PredefinedMenuItem::separator(),
		&quit_i,
	]);

	let mut tray_icon = TrayIconBuilder::new()
		.with_menu(Box::new(tray_menu))
		.with_tooltip("tao - awesome windowing lib")
		.with_icon(icon)
		.build()
		.ok();

	let menu_channel = MenuEvent::receiver();
	let tray_channel = TrayIconEvent::receiver();

	event_loop.run(move |_event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		if let Ok(event) = menu_channel.try_recv() {
			if event.id == quit_i.id() {
				tray_icon.take();
				*control_flow = ControlFlow::Exit;
			}
			println!("{event:?}");
		}

		if let Ok(event) = tray_channel.try_recv() {
			println!("{event:?}");
		}
	})
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
	let (icon_rgba, icon_width, icon_height) = {
		// let image = image::open(path).expect("Failed to open icon
		// path").into_rgba8();
		let image = image::ImageBuffer::from_fn(512, 32, |x, y| {
			if x % 2 == 0 {
				image::Rgba([0u8, 0u8, 0u8, 255u8])
			} else {
				image::Rgba([255u8, 0u8, 255u8, 255u8])
			}
		});

		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn main() {
	use log4rs::config::*;

	let log_appender = Appender::builder()
		.build("stdout".to_string(), Box::new(ConsoleAppender::builder().build()));

	let log_root = Root::builder().appender("stdout".to_string()).build(log::LevelFilter::Info);
	let log_config = Config::builder().appender(log_appender).build(log_root);

	init_config(log_config.unwrap()).unwrap();

	// let args = Args::parse();
	event_main();
	// app::App::default().run();
}
