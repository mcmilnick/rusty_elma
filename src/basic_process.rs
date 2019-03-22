use std::vec::Vec;
use crate::channel::Channel;
use crate::process::{ Process, StatusEnum};

/// # Arguments
/// 
/// * `_period` - The period at which the manager should call the update function of the process.
/// * `_previous_update` - This contains the the time that the previous update occured as is used to measure when the next update should occur in the manager.
/// * `_last_update` - Contains the time when the latest update happened. 
/// * `_start_time` - Contains the time when the start function was called for the process.
/// * `_name` - This contains the name of the process.
/// * `_num_updates` - A counter for the number of times the process has been updated since instantiation. 
/// * `_status` - Provides an unsigned byte showing the current state of the process. This enum is defined in the process file next to the trait. 
/// 
/// # Remarks
/// 
/// A basic process is one which doesn't have any special fields or do anything very specific. These are useful for quick testing.
pub struct BasicProcess {
    pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,
	pub _name : String,
	pub _num_updates : i64,
	pub _status : StatusEnum,
}

/// # Remarks
/// 
/// The implementation of a basic channel contains getters and setters for most of the fields in the BasicProcess struct. The only
/// field which is read only is the name. The update function given is very basic and takes care of the timing updates necessary to
/// keep the manager updating the process on a regular basis.
impl Process for BasicProcess {
    //getters
    fn period(&self)->std::time::Duration { return self._period }
    fn previous_update(&self)->std::time::Duration { return self._previous_update }
    fn last_update(&self)->std::time::Duration { return self._last_update }
    fn start_time(&self)->std::time::SystemTime { return self._start_time }
	fn name(&self)->&String { return &self._name }
    fn num_updates(&self)->i64 { return self._num_updates }
    fn status(&self)->&StatusEnum { return &self._status }
    
    //setters
    fn set_status(&mut self, status : StatusEnum) { self._status = status; }
    fn set_start_time(&mut self, st:std::time::SystemTime) { self._start_time = st; }
    fn set_prev_update(&mut self, pu:std::time::Duration) { self._previous_update = pu; }
    fn set_last_update(&mut self, lu:std::time::Duration) { self._last_update = lu; }
    fn set_num_update(&mut self, nu:i64) { self._num_updates = nu; }
    fn set_period(&mut self, per:std::time::Duration) { self._period = per; }

    //other functions
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {
        self.set_prev_update(self._last_update);
        self.set_last_update(_elapsed);
        self.set_num_update(self._num_updates + 1);
    }
}
