extern crate colored;

use manager::Manager;

pub enum StatusEnum { UNINITIALIZED, STOPPED, RUNNING }
	
pub struct Process {
	pub _period : std::time::Duration,
    pub _previous_update : std::time::Duration,
	pub _last_update : std::time::Duration,
	pub _start_time : std::time::Duration,

	pub _name : String,
	pub _num_updates : i64,

	pub _status : StatusEnum,
    //may need to use raw pointer
    pub _manager_ptr : Manager,
}
/*
	trait Process {
        virtual void init() = 0;
        virtual void start() = 0;
        virtual void update() = 0;
        virtual void stop() = 0;
	}

	impl Process {
		inline string name() { return _name; }
        inline status_type status() { return _status; }
        inline high_resolution_clock::duration period() { return _period; }
        inline int num_updates() { return _num_updates; }
        inline time_point<high_resolution_clock> start_time() { return _start_time; }
        inline high_resolution_clock::duration last_update() { return _last_update; }
        inline high_resolution_clock::duration previous_update() { return _previous_update; }

		//other getters
		Channel& channel(string name);
        double milli_time();
        double delta();
        string status_type_map() {
          switch(_status) {
            case UNINITIALIZED: return "UNINITIALIZED";
            case STOPPED: return "STOPPED";
            case RUNNING: return "RUNNING";
          }
		}

		// Manager interface
        void _init();
        void _start(high_resolution_clock::duration elapsed);
        void _update(high_resolution_clock::duration elapsed);
        void _stop();
	}
*/
