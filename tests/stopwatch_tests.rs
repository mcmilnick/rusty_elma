#[cfg(test)]
extern crate elma_builder;

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use elma_builder::stopwatch::Stopwatch;

#[test]
fn test_stopwatch() {
	let start = SystemTime::now();
	let mut s = Stopwatch {
	    sw_start_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_stop_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards"),
		sw_total_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	};
	Stopwatch::start(&mut s);
	let mut temp = Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	Stopwatch::reset(&mut s);
	temp = Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	Stopwatch::stop(&mut s);
	temp = Stopwatch::get_nano(&mut s); println!("{:?}", temp);
}
