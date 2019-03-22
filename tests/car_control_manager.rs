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
use std::num;

//////////////////////////////////////// open loop car example ///////////////////////
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
	elma.run(Duration::new(1,0) );
}


//////////////////////////////////////// closed loop car example ///////////////////////
pub struct ControllableCar {
	pub _velocity:f64,
	pub _force:f64,
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

impl process::Process for ControllableCar {
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
	}
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {
        self.set_prev_update(self._last_update);
        self.set_last_update(_elapsed);
        self.set_num_update(self._num_updates + 1);

		//grab from throttle
        let mut name_of_chan : String = "Throttle".to_string();
        let mut ind = 0;
        let mut chan_exists = false;
        for i in 0.._c.len() {
            if name_of_chan == _c[i].name().to_string() { ind = i; chan_exists = true; }
        }
        if true == chan_exists && _c[ind].nonempty() { self._force = _c[ind].latest_db(); }
		
		//update velocity
        self._velocity += ( self.delta() as f64 / 1000.0 ) * ( (self._k * -1.0) * self._velocity + self._force ) / self._m;

		//send new velocity
        name_of_chan = "Velocity".to_string();
        ind = 0;
        chan_exists = false;
        for i in 0.._c.len() {
            if name_of_chan == _c[i].name().to_string() { ind = i; chan_exists = true; }
        }
        if true == chan_exists { _c[ind].send(self._velocity); }

		print!("t: {} ms -- ", self.milli_time());
		print!("u: {} N -- ", self._force);
		println!("v: {} m/s", self._velocity);
    }
}



pub struct CruiseControl {
    pub _speed:f64,
    pub _desired_speed:f64,
    pub _KP:f64,
    pub _v : Vec<f64>,

    pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,
	pub _name : String,
	pub _num_updates : i64,
	pub _status : StatusEnum,
}

impl process::Process for CruiseControl {
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
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {
        self.set_prev_update(self._last_update);
        self.set_last_update(_elapsed);
        self.set_num_update(self._num_updates + 1);
		
		//grab new velocity
        let mut name_of_chan : String = "Velocity".to_string();
        let mut ind = 0;
        let mut chan_exists = false;
        for i in 0.._c.len() {
            if name_of_chan == _c[i].name().to_string() { ind = i; chan_exists = true; }
        }
        if true == chan_exists && _c[ind].nonempty() { self._speed = _c[ind].latest_db(); }
		
		//send new throttle
        name_of_chan = "Throttle".to_string();
        ind = 0;
        chan_exists = false;
        for i in 0.._c.len() {
            if name_of_chan == _c[i].name().to_string() { ind = i; chan_exists = true; }
        }
        if true == chan_exists {
			let calc = -self._KP*(self._speed - self._desired_speed);
			_c[ind].send(calc);
		}
    }
} 

#[test]
fn test_controllable_car() {
	let zero_time = Duration::new(0, 0);
	let mut car = ControllableCar {
		_velocity : 0.0,		_force : 0.0,
    	_k : 0.02,				_m : 1000.0,
		_period : Duration::new(0, 10_000_000),
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "Car".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,
	};
	let mut cc = CruiseControl {
		_speed : 0.0,
		_desired_speed : 50.0,
    	_KP : 314.15,
		_v : Vec::new(),
		_period : Duration::new(0, 10_000_000),
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "Control".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,	
	};
	let mut throttle = Channel {
        _name:"Throttle".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};
	let mut velocity = Channel {
        _name:"Velocity".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};

	/////////////////////// declare manager ////////////////////
	let mut elma = Manager {
		_processes : std::vec::Vec::new(),
    	_channels : std::vec::Vec::new(),
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	//schedule each process
	elma.schedule(&mut car, Duration::new(0, 10_000_000));
    elma.schedule(&mut cc, Duration::new(0, 10_000_000));
	//package the processes
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(car));
	procVec.push(Box::new(cc));
	//package the channels
	let mut chanVec : Vec<Box<Channel>> = Vec::new();
	chanVec.push(Box::new(throttle));
	chanVec.push(Box::new(velocity));
	//transfer ownership off all processes and channels to the manager
	elma.set_processes(procVec);
	elma.add_channel(chanVec);
	///////////////// run manager for 20 msecs ////////////////////
	elma.init();
	elma.run(Duration::new(10, 0) );
}
