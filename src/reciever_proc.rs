use std::vec::Vec;
use channel::Channel;

#[repr(u8)]
pub enum StatusEnum { UNINITIALIZED=0, STOPPED, RUNNING }

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

impl Reciever {
    pub fn period(&self)->std::time::Duration { return self._period }
    pub fn last_update(&self)->std::time::Duration { return self._last_update }
	pub fn name(&self)->&String { return &self._name }
    pub fn status(&self)->&StatusEnum { return &self._status }
    pub fn num_updates(&self)->i64 { return self._num_updates }
    pub fn start_time(&self)->std::time::SystemTime { return self._start_time }
    pub fn previous_update(&self)->std::time::Duration { return self._previous_update }
    pub fn milli_time(&self)->u64 {
        let temp_time = Reciever::last_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    pub fn status_type_map(&self)->String {
        match &self._status {
            UNINITIALIZED => return "UNINITIALIZED".to_string(),
            STOPPED => return "STOPPED".to_string(),
            RUNNING => return "RUNNING".to_string(),
        }
	}
    pub fn delta(&self)->u64 {
        let temp_time = Reciever::last_update(self) - Reciever::previous_update(self);
    	temp_time.as_secs() * 1000 as u64
    }

    //need to redo all below here

	// Manager interface
    pub fn _init(&mut self) {
        self._status = StatusEnum::STOPPED;
    }
    pub fn _start(&mut self, elapsed : std::time::Duration) {
        self._status = StatusEnum::RUNNING; 
        self._start_time = std::time::SystemTime::now();
        self._last_update = elapsed;
        self._num_updates = 0;
    }
    pub fn _update(&mut self, c : &mut Channel, elapsed : std::time::Duration) {
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
    pub fn _stop(&mut self) {
        self._status = StatusEnum::STOPPED;
    }

/////////////////// theoretically these would be virtual
    pub fn init() {}
    pub fn start() {}
    pub fn update(&mut self, c : &mut Channel) {}
    pub fn sum(&self)->f64 { return self._sum; }
    pub fn stop() {}
}
