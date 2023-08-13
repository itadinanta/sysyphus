use std::sync::atomic::*;
use std::sync::Arc;
use std::time::Duration;

use crate::clock::{Stopwatch, SystemTimer, Timer, TimerStopwatch};
use crate::gather::Sampler;

pub struct App {
	sampler: Sampler,
	interval: Duration,
	timer: SystemTimer,
}

impl Default for App {
	fn default() -> Self {
		App {
			sampler: Sampler::default(),
			interval: Duration::from_secs(1),
			timer: SystemTimer::new(),
		}
	}
}

impl App {
	pub fn tick(&mut self) {
		let now = self.timer.elapsed();
		let sample = self.sampler.sample();
		println!("{now:?}: {sample:?}");
	}

	pub fn run(&mut self) {
		let running = Arc::new(AtomicBool::new(true));
		let r = running.clone();

		ctrlc::set_handler(move || {
			r.store(false, Ordering::SeqCst);
		})
		.expect("Error setting Ctrl-C handler");

		let mut stopwatch = TimerStopwatch::new(&self.timer);
		while running.load(Ordering::SeqCst) {
			stopwatch.reset(&self.timer);
			self.tick();
			let elapsed_tick: Duration = stopwatch.elapsed(&self.timer);
			if elapsed_tick < self.interval {
				spin_sleep::sleep(self.interval - elapsed_tick);
			}
		}
	}
}
