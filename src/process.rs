use crate::channel::Channel;

#[repr(u8)]
pub enum StatusEnum { _uninitialized=0, _stopped, _running }

/// # Remarks
/// 
/// Note that the base implementation a process in Rust is through a trait. This is done to enforce certain rules
/// for all instantiations of a process. Ensure that all processes created utilize the process trait.
/// 
/// Functions given here are a mix of just a signature or may include the functionality. All functions in this trait are
/// available to use in the implementation of the trait, but one also has the opportunity to shadow any given function given
/// below in the implementation as long as it doesn't conflict with the fn signature. This means all given functions in this trait
/// are the equivalent of a  virtual function in C++
/// 
/// Of important note, if there is only a signature, that function needs expansion in the implementer. If the functionality is
/// called out below, then it doesn't need reimplementing in the implementation unless you want to overshadow it. We have thus provided
/// definete rules for all processes.
pub trait Process {
    // Grouping functions here which are only used for specific impl but still need defining in the trait
    fn sum(&self)->f64 { return 0.0; }
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {}


    // getters
    fn period(&self)->std::time::Duration;
    fn previous_update(&self)->std::time::Duration;
    fn last_update(&self)->std::time::Duration;
    fn start_time(&self)->std::time::SystemTime;
	fn name(&self)->&String;
    fn num_updates(&self)->i64;
    fn status(&self)->&StatusEnum;

    // setters
    fn set_status(&mut self, status : StatusEnum);
    fn set_start_time(&mut self, st:std::time::SystemTime);
    fn set_prev_update(&mut self, pu:std::time::Duration);
    fn set_last_update(&mut self, lu:std::time::Duration);
    fn set_num_update(&mut self, nu:i64);
    fn set_period(&mut self, per:std::time::Duration);

    // other functions used in all implementations
    fn milli_time(&self)->u64 {
    	(self.last_update().as_secs() * 1000 as u64) +
            (self.last_update().subsec_nanos() as u64) / 1000_000
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
        (temp_time.as_secs() * 1000 as u64) +
            (temp_time.subsec_nanos() as u64) / 1000_000
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
