#[cfg(test)]
extern crate elma_builder;
extern crate colored;

use colored::*;
use std::time::{SystemTime, Duration};
use std::collections::{VecDeque};
use elma_builder::process;
use elma_builder::process::{ StatusEnum, Process};
use elma_builder::channel::Channel;
use elma_builder::reciever_proc::Reciever;
use elma_builder::sender_proc::Sender;
use elma_builder::manager::Manager;
use std::vec::Vec;
use std::boxed;
use elma_builder::basic_process::BasicProcess;

pub struct OpenLoopCar {
	pub _velocity:f64,
    pub _k:f64,
    pub _m:f64,
    pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,
	pub _name : String,
	pub _num_updates : i64,
	pub _status : StatusEnum,
}

impl process::Process for OpenLoopCar {
    //functions just to grab basic data from the struct
    fn period(&self)->std::time::Duration { return self._period }
    fn previous_update(&self)->std::time::Duration { return self._previous_update }
    fn last_update(&self)->std::time::Duration { return self._last_update }
    fn start_time(&self)->std::time::SystemTime { return self._start_time }
	fn name(&self)->&String { return &self._name }
    fn num_updates(&self)->i64 { return self._num_updates }
    fn status(&self)->&StatusEnum { return &self._status }
    fn set_status(&mut self, status : StatusEnum) { self._status = status; }
    fn set_start_time(&mut self, st:std::time::SystemTime) { self._start_time = st; }
    fn set_prev_update(&mut self, pu:std::time::Duration) { self._previous_update = pu; }
    fn set_last_update(&mut self, lu:std::time::Duration) { self._last_update = lu; }
    fn set_num_update(&mut self, nu:i64) { self._num_updates = nu; }
    fn set_period(&mut self, per:std::time::Duration) { self._period = per; }

    //overshadows the traits method
	fn _start(&mut self, _elapsed : std::time::Duration) {
		Process::set_status(self, StatusEnum::_running);
        Process::set_start_time(self, std::time::SystemTime::now());
        Process::set_last_update(self, _elapsed);
        Process::set_num_update(self, 0);

		self._velocity = 0.0;
		self._k = 0.02;
        self._m = 100.0;
	}
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {
        self.set_prev_update(self._last_update);
        self.set_last_update(_elapsed);
        self.set_num_update(self._num_updates + 1);

		let u:f64 = 1.0;
        self._velocity += self.delta() as f64 * ( u - (self._k * self._velocity) ) / self._m;
		print!("t: {} ms -- ", self.milli_time());
		println!("v: {} m/s", self._velocity);
    }
}

#[test]
fn test_car_basic() {
	let zero_time = Duration::new(0, 0);
	let mut car = OpenLoopCar {
		_velocity : 10.0,
		_k : 0.02,
		_m : 100.0,
		_period : Duration::new(0, 10_000_000),
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "Toyota".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,
	};

	//package the processes
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(car));
	/////////////////////// declare manager ////////////////////
	let mut elma = Manager {
		_processes : procVec,
    	_channels : Vec::new(),
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	elma.init();
	elma.run(Duration::new(5,0) );
}
