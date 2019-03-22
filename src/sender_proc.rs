use std::vec::Vec;
use crate::channel::Channel;
use crate::process::*;

//virtual functions that were part of Process need to be implemented in their own mod with an instance of a process
pub struct Sender {
	pub _idx : usize,
    pub _data : Vec<f64>,
    pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,
	pub _name : String,
	pub _num_updates : i64,
	pub _status : StatusEnum,
}

impl Process for Sender {
    //functions just to grab basic data from the struct
    fn period(&self)->std::time::Duration { return self._period }
    fn previous_update(&self)->std::time::Duration { return self._previous_update }
    fn last_update(&self)->std::time::Duration { return self._last_update }
    fn start_time(&self)->std::time::SystemTime { return self._start_time }
	fn name(&self)->&String { return &self._name }
    fn num_updates(&self)->i64 { return self._num_updates }
    fn status(&self)->&StatusEnum { return &self._status }
    fn set_status(&mut self, status : StatusEnum) { self._status = status; }
    fn set_start_time(&mut self, st:std::time::SystemTime) { self._start_time = st; }
    fn set_prev_update(&mut self, pu:std::time::Duration) { self._previous_update = pu; }
    fn set_last_update(&mut self, lu:std::time::Duration) { self._last_update = lu; }
    fn set_num_update(&mut self, nu:i64) { self._num_updates = nu; }
    fn set_period(&mut self, per:std::time::Duration) { self._period = per; }

    //overshadows the traits method
    fn _update(&mut self, _c : &mut Vec<Box<Channel>>, _elapsed : std::time::Duration) {
        self._previous_update = self._last_update;
        self._last_update = _elapsed;
        self._num_updates =  self._num_updates + 1;

        let name_of_chan : String = "Data".to_string();
        let mut ind = 0;
        let mut chan_exists = false;
        for i in 0.._c.len() {
            if name_of_chan == _c[i].name().to_string() {
                ind = i;
                chan_exists = true;
            }
        }

        if true == chan_exists {
            _c[ind].send(self._data[self._idx]);
            self._idx = self._idx + 1;
            if self._idx==self._data.len() { self._idx=0; }
        }
    }
}
