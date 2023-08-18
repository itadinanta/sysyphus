///
use sysinfo::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Net {
	pub up: f32,
	pub down: f32,
}

#[derive(Copy, Clone, Default, Debug)]

pub struct Cpu {
	pub load: f32,
	pub sys: f32,
	pub idle: f32,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Mem {
	pub used: u64,
	pub free: u64,
}

#[derive(Clone, Default, Debug)]
pub struct Sample {
	pub cpu: Cpu,
	pub cpus: Vec<Cpu>,
	pub mem: Mem,
	pub net: Net,
	pub nics: Vec<Net>,
}

pub struct Sampler {
	sys: System,
}

impl Default for Sampler {
	fn default() -> Self { Sampler { sys: System::new_all() } }
}

impl Sampler {
	pub fn sample(&mut self) -> Sample {
		self.sys.refresh_all();
		let global_cpu = self.sys.global_cpu_info();
		let cpu = {
			let usage = global_cpu.cpu_usage() * 0.01;
			Cpu { idle: 1. - usage, load: usage, sys: 0. }
		};

		let cpus: Vec<Cpu> = self
			.sys
			.cpus()
			.iter()
			.map(|cpu| {
				let usage = cpu.cpu_usage() * 0.01;
				Cpu { idle: 1. - usage, load: usage, sys: 0. }
			})
			.collect();
		Sample { cpu, cpus, ..Default::default() }
	}
}
