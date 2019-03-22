extern crate colored;
#[cfg(test)]
extern crate elma_builder;

use colored::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::VecDeque;
use std::collections::HashMap;
use elma_builder::process::*;
use elma_builder::process;
use elma_builder::channel;
use elma_builder::channel::Channel;
use elma_builder::process::Process;
use elma_builder::reciever_proc::Reciever;
use elma_builder::sender_proc::Sender;
use elma_builder::manager;
use std::vec::Vec;
use std::boxed;

#[test]
fn test_manager_basic_refined() {
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

	//package the processes
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(sender));
	procVec.push(Box::new(reciever));
	//package the channels
	let mut chanVec : Vec<Box<channel::Channel>> = Vec::new();
	chanVec.push(Box::new(data));

	/////////////////////// declare manager ////////////////////
	let mut elma = manager::Manager {
		_processes : procVec,
    	_channels : chanVec,
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	///////////////// run manager for 10 seconds ////////////////////
	elma.init();
	elma.run(Duration::new(10, 0) );
}

//this test is a basic example which follows the same general API as the classic ELMA model.
//The test test_manager_basic_refined cuts out some unnecessary calls. I put both in just to demonstrate.
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

	/////////////////////// declare manager ////////////////////
	let mut elma = manager::Manager {
		_processes : std::vec::Vec::new(),
    	_channels : std::vec::Vec::new(),
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	//schedule each process
	elma.schedule(&mut sender, one_sec);
    elma.schedule(&mut reciever, one_sec);

	//package the processes
	let mut procVec : Vec<Box<Process>> = Vec::new();
	procVec.push(Box::new(sender));
	procVec.push(Box::new(reciever));
	//package the channels
	let mut chanVec : Vec<Box<Channel>> = Vec::new();
	chanVec.push(Box::new(data));

	//transfer ownership off all processes and channels to the manager
	elma.set_processes(procVec);
	elma.add_channel(chanVec);

	///////////////// run manager for 10 seconds ////////////////////
	elma.init();
	elma.run(Duration::new(10, 0) );
}
