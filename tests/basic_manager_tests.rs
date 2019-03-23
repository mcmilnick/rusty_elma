#[cfg(test)]
extern crate elma_builder;
extern crate colored;

use colored::*;
use std::time::{SystemTime, Duration};
use std::collections::{VecDeque};
use elma_builder::process;
use elma_builder::channel::Channel;
use elma_builder::reciever_proc::Reciever;
use elma_builder::sender_proc::Sender;
use elma_builder::manager::Manager;
use std::vec::Vec;
use std::boxed;
use elma_builder::basic_process::BasicProcess;

#[test]
fn test_manager_basic() {
	//when testing, you can pass cargo test -- --nocapture to see printout of realtime values
	/////////////////////// setup basic data and test types ////////////////////
	let one_msec = Duration::new(0, 1000_000);
	let zero_time = Duration::new(0, 0);

	let sendvec = vec![1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0];
	let mut sender = Sender {
		_idx : 0,
		_data : sendvec,
		_period : one_msec,
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "sender".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,
	};
	let mut reciever = Reciever {
		_sum : 0.0,
		_period : one_msec,
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "reciever".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,	
	};
	let mut data = Channel {
        _name:"Data".to_string(),
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
	elma.schedule(&mut sender, one_msec);
    elma.schedule(&mut reciever, one_msec);
	//package the processes
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(sender));
	procVec.push(Box::new(reciever));
	//package the channels
	let mut chanVec : Vec<Box<Channel>> = Vec::new();
	chanVec.push(Box::new(data));
	//transfer ownership off all processes and channels to the manager
	elma.set_processes(procVec);
	elma.add_channel(chanVec);
	///////////////// run manager for 20 msecs ////////////////////
	elma.init();
	elma.run(Duration::new(0, 20_000_000) );

	//final test results
	let data = elma._channels[0].latest_vec(elma._channels[0].size());
	let sum : f64 = data.iter().sum(); 
	assert_eq!(45.0, sum);
}

#[test]
fn test_manager_updates() {
	let zero_time = Duration::new(0, 0);
	let mut pProc = BasicProcess {
		_period : Duration::new(0, 1000_000),
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "p".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,
	};
	let mut qProc = BasicProcess {
		_period : Duration::new(0, 5000_000),
		_previous_update : zero_time,
		_last_update : zero_time,
		_start_time : SystemTime::now(),
		_name : "q".to_string(),
		_num_updates : 0,
		_status : process::StatusEnum::_uninitialized,	
	};

	//package the processes
	let mut procVec : Vec<Box<process::Process>> = Vec::new();
	procVec.push(Box::new(pProc));
	procVec.push(Box::new(qProc));
	/////////////////////// declare manager ////////////////////
	let mut elma = Manager {
		_processes : procVec,
    	_channels : Vec::new(),
    	_start_time : zero_time,
		_elapsed : zero_time,
	};

	elma.init();
	elma.run(Duration::new(0, 11_000_000) );

	assert_eq!(elma._processes[0].num_updates(), 10);
    assert_eq!(elma._processes[1].num_updates(), 2);
}
