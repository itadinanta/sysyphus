use std::fmt;
use std::time;
use std::time::Duration;

/// Timer
pub trait Timer {
	fn elapsed(&self) -> Duration;
}

/// SystemTimer
#[derive(Clone)]
pub struct SystemTimer {
	t0: time::SystemTime,
}

impl Default for SystemTimer {
	fn default() -> Self { SystemTimer { t0: time::SystemTime::now() } }
}

impl SystemTimer {
	pub fn new() -> Self { SystemTimer::default() }
}

impl Timer for SystemTimer {
	fn elapsed(&self) -> Duration {
		match self.t0.elapsed() {
			Ok(dt) => dt,
			Err(_) => Duration::ZERO,
		}
	}
}

/// Stopwatch
pub trait Stopwatch {
	fn reset<T>(&mut self, timer: &T)
	where T: Timer;

	fn elapsed<T>(&self, timer: &T) -> Duration
	where T: Timer;

	fn restart<T>(&mut self, timer: &T) -> Duration
	where T: Timer {
		let elapsed = self.elapsed(timer);
		self.reset(timer);
		elapsed
	}
}

#[derive(Clone)]
pub struct TimerStopwatch {
	t0: Duration,
}

impl TimerStopwatch {
	pub fn new(timer: &dyn Timer) -> Self {
		let t0 = timer.elapsed();
		TimerStopwatch { t0 }
	}
}

impl Stopwatch for TimerStopwatch {
	fn reset<T>(&mut self, timer: &T)
	where T: Timer {
		self.t0 = timer.elapsed();
	}

	fn elapsed<T>(&self, timer: &T) -> Duration
	where T: Timer {
		timer.elapsed() - self.t0
	}
}

/// Hourglass
#[derive(Clone)]
pub struct Hourglass {
	stopwatch: TimerStopwatch,
	capacity: Duration,
	timeout: Duration,
}

impl fmt::Debug for Hourglass {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.timeout.as_secs_f64(), self.capacity.as_secs_f64())
	}
}

#[allow(unused)]
impl Hourglass {
	pub fn new(elapsed: Duration, timer: &dyn Timer) -> Self {
		Hourglass { stopwatch: TimerStopwatch::new(timer), capacity: elapsed, timeout: elapsed }
	}

	pub fn renew<T>(&mut self, timer: &T)
	where T: Timer {
		self.timeout = self.capacity;
		self.stopwatch.reset(timer)
	}

	pub fn flip<T>(&mut self, timer: &T) -> Duration
	where T: Timer {
		let left = self.left(timer);
		self.timeout = self.capacity - left;
		self.stopwatch.reset(timer);
		left
	}

	pub fn delay(&mut self, delay_seconds: Duration) { self.timeout += delay_seconds; }

	#[allow(unused)]
	pub fn elapsed<T>(&self, timer: &T) -> Duration
	where T: Timer {
		self.stopwatch.elapsed(timer)
	}

	pub fn left<T>(&self, timer: &T) -> Duration
	where T: Timer {
		let dt = self.timeout - self.stopwatch.elapsed(timer);
		Duration::max(Duration::ZERO, dt)
	}

	pub fn is_expired<T>(&self, timer: &T) -> bool
	where T: Timer {
		self.left(timer) <= Duration::ZERO
	}

	pub fn flip_if_expired<T>(&mut self, timer: &T) -> bool
	where T: Timer {
		let expired = self.is_expired(timer);
		if expired {
			self.flip(timer);
		};
		expired
	}
}
