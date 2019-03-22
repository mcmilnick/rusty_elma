extern crate colored;
#[cfg(test)]
extern crate elma_builder;

use colored::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::VecDeque;
use std::collections::HashMap;
use elma_builder::process::*;
use std::vec::Vec;
use std::boxed;
use elma_builder::channel::Channel;

#[test]
fn test_channel() {
    //should be able to test now
	let mut data = Channel {
        _name:"Data".to_string(),
        _capacity:10,
        _queue:VecDeque::new(),
	};

	let test_str = "Data".to_string();
	let test_cap:usize = 10;
	assert_eq!(Channel::size(&data), 0);
	assert_eq!(Channel::empty(&data), true);
	assert_eq!(Channel::nonempty(&data), false);
	assert_eq!(Channel::name(&data), &test_str);
	assert_eq!(Channel::capacity(&data), &test_cap);

	let mut test_vec:VecDeque<f64> = VecDeque::new();
	test_vec.push_back(1.0);
	test_vec.push_back(2.0);
	test_vec.push_back(3.0);
	test_vec.push_back(4.0);
	Channel::send(&mut data, 1.0);
	Channel::send(&mut data, 2.0);
	Channel::send(&mut data, 3.0);
	Channel::send(&mut data, 4.0);
	assert_eq!(Channel::size(&data), 4);

	let mut test_val:f64 = 4.0;
	assert_eq!(Channel::capacity(&data), &test_cap);
	assert_eq!(Channel::latest_db(&data), test_val);
	test_val = 1.0;
	assert_eq!(Channel::earliest(&data), test_val);
}
