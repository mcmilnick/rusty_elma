extern crate colored;

use colored::*;
use std::time::{SystemTime, Instant, UNIX_EPOCH};

fn gcd(mut n: u64, mut m: u64) -> u64 { assert!(n!=0 &&	m!=0);
    while m!=0 {
        if m < n {
		    let t = m;
			m = n;
			n =	t;
		}
		m = m%n;
	}
	n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
	assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

#[test]
fn test_stopwatch() {
    assert_eq!(gcd(14,15),1);
	assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
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
	
	let start = SystemTime::now();
	let mut s = stopwatch_mod::Stopwatch {
	    sw_start_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	    ,sw_stop_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	    ,sw_total_time:start.duration_since(UNIX_EPOCH).expect("Time went backwards")
	};
	stopwatch_mod::Stopwatch::start(&mut s);
	let mut temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch_mod::Stopwatch::reset(&mut s);
	temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	stopwatch_mod::Stopwatch::stop(&mut s);
	temp = stopwatch_mod::Stopwatch::get_nano(&mut s); println!("{:?}", temp);
	
	println!("{}", "App end".green());
	
}
