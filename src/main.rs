extern crate colored;

use colored::*;
use std::time::{SystemTime, Instant, UNIX_EPOCH};
use std::collections::VecDeque;
use std::collections::HashMap;

mod process;
mod channel;
mod manager;

#[test]
fn test_channel() {
    //should be able to test now
	let mut data = channel::Channel {
        _name:"Data".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};

	let test_str = "Data".to_string();
	let test_cap:usize = 10;
	assert_eq!(channel::Channel::size(&data), 0);
	assert_eq!(channel::Channel::empty(&data), true);
	assert_eq!(channel::Channel::nonempty(&data), false);
	assert_eq!(channel::Channel::name(&data), &test_str);
	assert_eq!(channel::Channel::capacity(&data), &test_cap);

	let mut test_vec:VecDeque<f64> = VecDeque::new();
	test_vec.push_back(1.0);
	test_vec.push_back(2.0);
	test_vec.push_back(3.0);
	test_vec.push_back(4.0);
	channel::Channel::send(&mut data, 1.0);
	channel::Channel::send(&mut data, 2.0);
	channel::Channel::send(&mut data, 3.0);
	channel::Channel::send(&mut data, 4.0);
	assert_eq!(channel::Channel::size(&data), 4);

	let mut test_val:f64 = 4.0;
	assert_eq!(channel::Channel::capacity(&data), &test_cap);
	assert_eq!(channel::Channel::latest_db(&data), test_val);
	test_val = 1.0;
	assert_eq!(channel::Channel::earliest(&data), test_val);
}

//be aware I am throwing out the pointer system for manager pointer and need a new manager runner class
//also need replacement virtual classes from process in new mod
#[test]
fn test_manager() {
	/* need process for reciever and sender
    m.schedule(sender, MS(10))
    .schedule(receiver, MS(10))
    .add_channel(data)
    .init();
    for(int i=0; i<ans.size(); i++){
        m.run(MS(20));
        EXPECT_DOUBLE_EQ(receiver.sum(), ans[i]);
    }
	*/
	let starter = std::time::SystemTime::now();
	let dur_temp = starter.duration_since(std::time::UNIX_EPOCH)
	    	.expect("Time went backwards");
	let mut elma = manager::Manager {
		_processes : std::vec::Vec::new(),
    	_channels : HashMap::<String, &channel::Channel>::new(),
    	_start_time : dur_temp,
		_elapsed : dur_temp,
	};
	let mut sender = process::Process {
		_period : dur_temp,
		_previous_update : dur_temp,
		_last_update : dur_temp,
		_start_time : std::time::SystemTime::now(),
		_name : "sender".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::UNINITIALIZED,
	};
	let mut reciever = process::Process {
		_period : dur_temp,
		_previous_update : dur_temp,
		_last_update : dur_temp,
		_start_time : std::time::SystemTime::now(),
		_name : "reciever".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::UNINITIALIZED,	
	};
	let mut data = channel::Channel {
        _name:"Data".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};

	//init, start, update, stop need done from here since we can not have nested self refs in rust
	//may be worth adding into manager later by declaring the actual processes instead of references:
	//I may store the processes as traits
	elma._elapsed = starter.duration_since(std::time::UNIX_EPOCH)
	    .expect("Time went backwards");
	process::Process::_init(&mut sender);
	process::Process::_init(&mut reciever); 

    elma._start_time = starter.duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
	process::Process::_start(&mut sender, elma._elapsed);
	process::Process::_start(&mut reciever, elma._elapsed);

	let runtime = std::time::Duration::new(20, 0);;
    while elma._elapsed < runtime {
		if elma._elapsed > process::Process::last_update(&sender) + process::Process::period(&sender) {
			process::Process::_update(&mut sender, elma._elapsed);
		};
		if elma._elapsed > process::Process::last_update(&reciever) + process::Process::period(&reciever) {
			process::Process::_update(&mut sender, elma._elapsed);
		};

		let temp = std::time::SystemTime::now();
		elma._elapsed = temp.duration_since(std::time::UNIX_EPOCH)
			.expect("Time went backwards")
			- elma._start_time;
    }

    process::Process::_stop(&mut sender);
	process::Process::_stop(&mut reciever);
}

#[test]
fn test_stopwatch() {
	let start = SystemTime::now();
	let mut s = stopwatch_mod::Stopwatch {
	    sw_start_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_stop_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_total_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	};
	stopwatch_mod::Stopwatch::start(&mut s);
	let mut temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch_mod::Stopwatch::reset(&mut s);
	temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch_mod::Stopwatch::stop(&mut s);
	temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
}

mod stopwatch_mod {
    pub struct Stopwatch {
        pub sw_start_time: std::time::Duration,
        pub sw_stop_time: std::time::Duration,
        pub sw_total_time: std::time::Duration,
	}

	impl Stopwatch {
        pub fn start(&mut self) {
		    let temp = std::time::SystemTime::now();
            self.sw_start_time = temp.duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
	        self.sw_stop_time = temp.duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
        }
        pub fn stop(&mut self) {
            if self.sw_stop_time == self.sw_start_time {
			    let temp = std::time::SystemTime::now();
                self.sw_stop_time = temp.duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                self.sw_total_time += self.sw_stop_time - self.sw_start_time;
            }
        }
        pub fn reset(&mut self) {
		    let temp = std::time::SystemTime::now();
			
		    //if running, reset both the start and stop time
		    if self.sw_stop_time == self.sw_start_time {
                self.sw_start_time = temp.duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
	            self.sw_stop_time = temp.duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
			}
			
            self.sw_total_time = std::time::Duration::new(0, 0);
        }
  
        pub fn get_sec(&self) -> u64 {
		    if self.sw_stop_time == self.sw_start_time {
			    let temp = std::time::SystemTime::now();
                let temp_time = temp.duration_since(std::time::UNIX_EPOCH)
				    .expect("Time went backwards") -
				    self.sw_start_time;
    	        temp_time.as_secs()
			} else {
			    self.sw_total_time.as_secs()
			}
	    }
        pub fn get_milli(&self) -> u64 {
		    if self.sw_stop_time == self.sw_start_time {
			    let temp = std::time::SystemTime::now();
                let temp_time = temp.duration_since(std::time::UNIX_EPOCH)
				    .expect("Time went backwards") -
				    self.sw_start_time;
    	        temp_time.as_secs() * 1000 +
	                self.sw_total_time.subsec_nanos() as u64 / 1_000_000
			} else {
			    self.sw_total_time.as_secs() * 1000 +
	                self.sw_total_time.subsec_nanos() as u64 / 1_000_000
			}
	    }
        pub fn get_nano(&self) -> u64 {
		    if self.sw_stop_time == self.sw_start_time {
			    let temp = std::time::SystemTime::now();
                let temp_time = temp.duration_since(std::time::UNIX_EPOCH)
				    .expect("Time went backwards") -
				    self.sw_start_time;
    	        temp_time.as_secs() * 1000_000_000 +
	                self.sw_total_time.subsec_nanos() as u64
			} else {
			    self.sw_total_time.as_secs() * 1000_000_000 +
	                self.sw_total_time.subsec_nanos() as u64
			}
	    }
    }
}

fn main() {
    println!("{}", "App start".green());
	

	
	println!("{}", "App end".green());
	
}
