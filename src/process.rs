use channel::Channel;

#[repr(u8)]
pub enum StatusEnum { _uninitialized=0, _stopped, _running }

//traits can't access fields, so we must suplicate the getters and setters in each impl
pub trait Process {
    /////////////////// Any functions specific to certain impl /////////////////////////
    fn sum(&self)->f64 { return 0.0; }
    fn _update(&mut self, _c : &mut Channel, _elapsed : std::time::Duration) {}


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
    fn set_period(&mut self, per:std::time::Duration);

    ///////////////// Functions which should be good over all impl using Process ///////////////////
    fn milli_time(&self)->u64 {
        let temp_time = Process::last_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    fn status_type_map(&self)->String {
        match Process::status(self) {
            StatusEnum::_uninitialized => return "_uninitialized".to_string(),
            StatusEnum::_stopped => return "_stopped".to_string(),
            StatusEnum::_running => return "_running".to_string(),
        }
	}
    fn delta(&self)->u64 {
        let temp_time = Process::last_update(self) - Process::previous_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    fn _init(&mut self) {
        Process::set_status(self, StatusEnum::_stopped);
    }
    fn _stop(&mut self) {
        Process::set_status(self, StatusEnum::_stopped);
    }
    fn _start(&mut self, _elapsed : std::time::Duration) {
        Process::set_status(self, StatusEnum::_running);
        Process::set_start_time(self, std::time::SystemTime::now());
        Process::set_last_update(self, _elapsed);
        Process::set_num_update(self, 0);
    }
}
