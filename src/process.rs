use channel::Channel;

#[repr(u8)]
pub enum StatusEnum { UNINITIALIZED=0, STOPPED, RUNNING }

pub trait Process {
    /////////////////// Any functions specific to certain impl /////////////////////////
    fn sum(&self)->f64 { return 0.0; }
    fn _update(&mut self, c : &mut Channel, elapsed : std::time::Duration) {}


    /////////////////// Basic functions to grab constant struct data /////////////////////////////
    fn period(&self)->std::time::Duration;
    fn previous_update(&self)->std::time::Duration;
    fn last_update(&self)->std::time::Duration;
    fn start_time(&self)->std::time::SystemTime;
	fn name(&self)->&String;
    fn num_updates(&self)->i64;
    fn status(&self)->&StatusEnum;
    fn set_status(&mut self, status : StatusEnum);
    fn set_start_time(&mut self, st:std::time::SystemTime);
    fn set_prev_update(&mut self, pu:std::time::Duration);
    fn set_last_update(&mut self, lu:std::time::Duration);
    fn set_num_update(&mut self, nu:i64);

    ///////////////// Functions which should be good over all impl using Process ///////////////////
    fn milli_time(&self)->u64 {
        let temp_time = Process::last_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    fn status_type_map(&self)->String {
        match Process::status(self) {
            UNINITIALIZED => return "UNINITIALIZED".to_string(),
            STOPPED => return "STOPPED".to_string(),
            RUNNING => return "RUNNING".to_string(),
        }
	}
    fn delta(&self)->u64 {
        let temp_time = Process::last_update(self) - Process::previous_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    fn _init(&mut self) {
        Process::set_status(self, StatusEnum::STOPPED);
    }
    fn _stop(&mut self) {
        Process::set_status(self, StatusEnum::STOPPED);
    }
    fn _start(&mut self, elapsed : std::time::Duration) {
        Process::set_status(self, StatusEnum::RUNNING);
        Process::set_start_time(self, std::time::SystemTime::now());
        Process::set_last_update(self, elapsed);
        Process::set_num_update(self, 0);
    }
}
