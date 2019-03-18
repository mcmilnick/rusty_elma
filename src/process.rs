extern crate colored;

use manager::Manager;

#[repr(u8)]
pub enum StatusEnum { UNINITIALIZED=0, STOPPED, RUNNING }
	
pub struct Process {
	pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::Duration,

	pub _name : String,
	pub _num_updates : i64,

	pub _status : StatusEnum,
    //may need to use raw pointer
    //pub _manager_ptr : &'man: 'a Manager,
}

/*
trait Process {
    fn init() {}
    fn start() {}
    fn update() {}
    fn stop() {}
}
*/

impl Process {
    pub fn period(&self)->std::time::Duration { return self._period }
    pub fn last_update(&self)->std::time::Duration { return self._last_update }
	pub fn name(&self)->&String { return &self._name }
    pub fn status(&self)->&StatusEnum { return &self._status }
    pub fn num_updates(&self)->i64 { return self._num_updates }
    pub fn start_time(&self)->std::time::Duration { return self._start_time }
    pub fn previous_update(&self)->std::time::Duration { return self._previous_update }
    pub fn milli_time(&self)->u64 {
		let temp = std::time::SystemTime::now();
        let temp_time = temp.duration_since(std::time::UNIX_EPOCH)
			.expect("Time went backwards") -
		    Process::last_update(self);
    	temp_time.as_secs() * 1000 as u64
    }
    pub fn status_type_map(&self)->String {
        match &self._status {
            UNINITIALIZED => return "UNINITIALIZED".to_string(),
            STOPPED => return "STOPPED".to_string(),
            RUNNING => return "RUNNING".to_string(),
        }
	}

    //need to redo all below here
    //Channel& channel(string name);
    pub fn delta()->f64 { return 0.0}

	// Manager interface
    pub fn _init() {}
    pub fn _start(elapsed : std::time::Duration) {}
    pub fn _update(elapsed : std::time::Duration) {}
    pub fn _stop() {}
}

/*
    Channel& Process::channel(string name) {
        return _manager_ptr->channel(name);
    }

    double Process::milli_time() {
        duration<double, std::milli> time = last_update();
        return time.count();
    }
    double Process::delta() { 
        duration<double, std::milli> diff = last_update() - previous_update();
        return diff.count();
    }

    // Manager interface
    void Process::_init() { 
        _status = STOPPED;     
        init();
    }
    void Process::_start(high_resolution_clock::duration elapsed) { 
        _status = RUNNING; 
        _start_time = high_resolution_clock::now();
        _last_update = elapsed;
        _num_updates = 0;
        start();
    }
    void Process::_update(high_resolution_clock::duration elapsed) {
        _previous_update = _last_update;
        _last_update = elapsed;
        update();
        _num_updates++;
    }
    void Process::_stop() { 
        _status = STOPPED; 
        stop();
    }   
*/