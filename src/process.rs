extern crate colored;

use manager::Manager;

#[repr(u8)]
pub enum StatusEnum { UNINITIALIZED=0, STOPPED, RUNNING }

//virtual functions that were part of Process need to be implemented in their own mod with an instance of a process
pub struct Process {
	pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::SystemTime,

	pub _name : String,
	pub _num_updates : i64,

	pub _status : StatusEnum,
}

impl Process {
    pub fn period(&self)->std::time::Duration { return self._period }
    pub fn last_update(&self)->std::time::Duration { return self._last_update }
	pub fn name(&self)->&String { return &self._name }
    pub fn status(&self)->&StatusEnum { return &self._status }
    pub fn num_updates(&self)->i64 { return self._num_updates }
    pub fn start_time(&self)->std::time::SystemTime { return self._start_time }
    pub fn previous_update(&self)->std::time::Duration { return self._previous_update }
    pub fn milli_time(&self)->u64 {
        let temp_time = Process::last_update(self);
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
        let temp_time = Process::last_update(self) - Process::previous_update(self);
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
    pub fn _update(&mut self, elapsed : std::time::Duration) {
        self._previous_update = self._last_update;
        self._last_update = elapsed;
        self._num_updates =  self._num_updates + 1;
    }
    pub fn _stop(&mut self) {
        self._status = StatusEnum::STOPPED;
    }

    //Channel& channel(string name) { return _manager_ptr->channel(name); }
}
