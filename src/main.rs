extern crate colored;

use colored::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::VecDeque;
use std::collections::HashMap;
use process::*;
use reciever_proc::Reciever;
use sender_proc::Sender;
use std::vec::Vec;
use std::boxed;

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
	/////////////////////// setup basic data and test types ////////////////////
	let one_sec = Duration::new(1, 0);
	let zero_time = Duration::new(0, 0);

	let sendvec = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
	let mut sender = Sender {
		_idx : 0,
		_data : sendvec,
		_period : one_sec,
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "sender".to_string(),
		_num_updates : 0,
		_status : StatusEnum::_uninitialized,
	};
	let mut reciever = Reciever {
		_n : 0,
		_sum : 0.0,
		_period : one_sec,
		_previous_update : zero_time,
		_last_update : zero_time,
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

	/////////////////////// setup manager ////////////////////
	let mut elma = manager::Manager {
		_processes : std::vec::Vec::new(),
    	_channels : std::vec::Vec::new(),
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	elma.schedule(&mut sender, one_sec);
    elma.schedule(&mut reciever, one_sec);
	elma.add_channel(&mut data);

	//need to make the processes and channels into boxes to send into the manager
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(sender));
	procVec.push(Box::new(reciever));
	
	let mut chanVec : Vec<Box<channel::Channel>> = Vec::new();
	chanVec.push(Box::new(data));

	///////////////// run manager ////////////////////
	elma.init(&mut procVec);
	println!("{}", "manager starting".green());
	elma.start(&mut procVec);
	elma.run(&mut procVec, &mut chanVec, 10);
	elma.stop(&mut procVec);
	println!("{}", "maanager stopped".green());
}

fn main() {
	
}
