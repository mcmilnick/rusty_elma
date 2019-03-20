use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[allow(dead_code)]
pub struct Stopwatch {
    pub sw_start_time: Duration,
    pub sw_stop_time: Duration,
    pub sw_total_time: Duration,
}

#[allow(dead_code)]
impl Stopwatch {
    pub fn start(&mut self) {
	    let temp = SystemTime::now();
        self.sw_start_time = temp.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        self.sw_stop_time = temp.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    }
    pub fn stop(&mut self) {
        if self.sw_stop_time == self.sw_start_time {
			let temp = SystemTime::now();
            self.sw_stop_time = temp.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            self.sw_total_time += self.sw_stop_time - self.sw_start_time;
        }
    }
    pub fn reset(&mut self) {
		let temp = SystemTime::now();
			
	    //if running, reset both the start and stop time
	    if self.sw_stop_time == self.sw_start_time {
            self.sw_start_time = temp.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
	        self.sw_stop_time = temp.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
		}
			
        self.sw_total_time = Duration::new(0, 0);
    }
  
    pub fn get_sec(&self) -> u64 {
		if self.sw_stop_time == self.sw_start_time {
		    let temp = SystemTime::now();
            let temp_time = temp.duration_since(UNIX_EPOCH)
				.expect("Time went backwards") -
			    self.sw_start_time;
    	    temp_time.as_secs()
		} else {
		    self.sw_total_time.as_secs()
		}
	}
    pub fn get_milli(&self) -> u64 {
		if self.sw_stop_time == self.sw_start_time {
		    let temp = SystemTime::now();
            let temp_time = temp.duration_since(UNIX_EPOCH)
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
			let temp = SystemTime::now();
            let temp_time = temp.duration_since(UNIX_EPOCH)
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
