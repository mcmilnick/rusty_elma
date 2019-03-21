extern crate colored;

use colored::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::VecDeque;
use std::collections::HashMap;
use process::*;
use reciever_proc::Reciever;
use sender_proc::Sender;
use std::vec::Vec;

mod process;
mod channel;
mod manager;
mod stopwatch;
mod sender_proc;
mod reciever_proc;

/////////////////////////// finished tests /////////////////////////
#[test]
fn test_stopwatch() {
	let start = SystemTime::now();
	let mut s = stopwatch::Stopwatch {
	    sw_start_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_stop_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_total_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	};
	stopwatch::Stopwatch::start(&mut s);
	let mut temp = stopwatch::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch::Stopwatch::reset(&mut s);
	temp = stopwatch::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch::Stopwatch::stop(&mut s);
	temp = stopwatch::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
}

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

#[test]
fn test_manager_basic() {
	//when testing, you can pass cargo test -- --nocapture to see printout of realtime values
	let starter = SystemTime::now();
	let temp_time = starter.duration_since(UNIX_EPOCH).expect("Time went backwards");
	let one_sec = Duration::new(1, 0);

	let mut elma = manager::Manager {
		_processes : std::vec::Vec::new(),
    	_channels : std::vec::Vec::new(),
    	_start_time : temp_time,
		_elapsed : temp_time,
	};

	let sendvec = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
	let mut sender = Sender {
		_idx : 0,
		_data : sendvec,
		_period : one_sec,
		_previous_update : temp_time,
		_last_update : temp_time,
		_start_time : SystemTime::now(),
		_name : "sender".to_string(),
		_num_updates : 0,
		_status : StatusEnum::_uninitialized,
	};
	let mut reciever = Reciever {
		_n : 0,
		_sum : 0.0,
		_period : one_sec,
		_previous_update : temp_time,
		_last_update : temp_time,
		_start_time : SystemTime::now(),
		_name : "reciever".to_string(),
		_num_updates : 0,
		_status : StatusEnum::_uninitialized,	
	};
	let mut data = channel::Channel {
        _name:"Data".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};

	//let processVec = Vec::new();
	//processVec.push(&sender);
	//processVec.push(&reciever);

	elma.schedule(&mut sender, one_sec);
	elma.schedule(&mut reciever, one_sec);
	elma.add_channel(&mut data);
	//Sender::_init(&mut sender);
	//Reciever::_init(&mut reciever); 

    elma._start_time = starter.duration_since(UNIX_EPOCH).expect("Time went backwards");
	elma._elapsed = starter.duration_since(UNIX_EPOCH).expect("Time went backwards") - elma._start_time;

	Sender::_start(&mut sender, elma._elapsed);
	Reciever::_start(&mut reciever, elma._elapsed);

	println!("{}", "manager start".green());
	let runtime = Duration::new(20, 0);
    while elma._elapsed < runtime {
		if elma._elapsed > Sender::last_update(&sender) + Sender::period(&sender) {
			Sender::_update(&mut sender, &mut data, elma._elapsed);
		};
		if elma._elapsed > Reciever::last_update(&reciever) + Reciever::period(&reciever) {
			Reciever::_update(&mut reciever, &mut data, elma._elapsed);
		};

		let temp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
		elma._elapsed = temp - elma._start_time;
    }

    Sender::_stop(&mut sender);
	Reciever::_stop(&mut reciever);
	println!("{}", "maanager stop".green());
}

fn main() {
	
}
