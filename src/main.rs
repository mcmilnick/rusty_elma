extern crate colored;

use colored::*;
use std::time::{SystemTime, Instant, UNIX_EPOCH};

mod process;
mod channel;

#[test]
fn test_channel() {
    //should be able to test now
}

#[test]
fn test_manager() {
	let ans:std::vec::Vec<f64> = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55];
	/* need process for reciever and sender
    elma::Manager m;
    Sender sender("sender", vector<double>{1, 2, 3, 4, 5, 6, 7, 8, 9, 10});
    Receiver receiver("receiver", 10);
    elma::Channel data("Data");

    m.schedule(sender, MS(10))
    .schedule(receiver, MS(10))
    .add_channel(data)
    .init();
    for(int i=0; i<ans.size(); i++){
        m.run(MS(20));
        EXPECT_DOUBLE_EQ(receiver.sum(), ans[i]);
    }
	*/
}

#[test]
fn test_stopwatch() {
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
