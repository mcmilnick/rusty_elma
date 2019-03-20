use std::vec::Vec;
use channel::Channel;
use process::*;

//virtual functions that were part of Process need to be implemented in their own mod with an instance of a process
pub struct Reciever {
	pub _n : usize,
    pub _sum : f64,
    pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,
	pub _name : String,
	pub _num_updates : i64,
	pub _status : StatusEnum,
}

impl Process for Reciever {
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

    //overshadows the traits method
    fn _update(&mut self, c : &mut Channel, elapsed : std::time::Duration) {
        self._previous_update = self._last_update;
        self._last_update = elapsed;
        self._num_updates =  self._num_updates + 1;

        if(c.nonempty()){
            self._sum = 0.0;
            let data = c.latest_vec(c.size());
            self._sum = data.iter().sum(); 
            println!("recieved sum is: {:?}", self._sum);
        }
    }
    fn sum(&self)->f64 { return self._sum; }
}