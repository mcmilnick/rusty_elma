use std::vec::Vec;
use std::collections::HashMap;
use crate::process::Process;
use crate::channel::Channel;
use std::boxed;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// # Arguments
/// 
/// * `_processes` - When a process is created and then scheduled, the ownership is transferred to the manager and the process is stored in this vector.
/// * `_channels` - When a channel is created and then added to the manager it is stored in this vector.
/// * `_start_time` - This contains the time at which the manager was started. 
/// * `_elapsed` - This field contains the amount of time which has passed since the manager was started.

/// # Remarks
/// 
/// The manager struct contains data which will interact with the processes and channels given to it. This struct will gain ownership of the data given to it
/// and this will be stored in the _processes and _channels fields. These fields can then lend ownership to subfunctions within the manager, but can not
/// give up ownership.
#[allow(dead_code)]
pub struct Manager {
    pub _processes : Vec<Box<Process>>,
    pub _channels : Vec<Box<Channel>>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

/// # Remarks
/// 
/// The implementation of the manager contains knowledge of how and when to operate on the given processes and channels owned by the manager.
/// The schedule fn is how to pass ownership of processes to the manager and the add_channel fn gives ownership of channels to the manager.
/// The user passes a run time to the manager, which then sits in a tight loop until this amount of time has passed. After general setup, using the
/// run fn will start and continue updates until finally calling the stop fn.
/// The main update process occuring in the run loop looks for the scheduled timing of individual processes and calls their corresponding update fn when
/// appropriate. These sub processes also interact with different channels so it is important to lend ownership of the channels to the process update channels
/// when calling them.
#[allow(dead_code)]
impl Manager {
    pub fn set_processes(&mut self, vb : Vec<Box<Process>>) {
        self._processes = vb;
    }
    pub fn start_time(&self)->std::time::Duration { self._start_time }
    pub fn elapsed(&self)->std::time::Duration { self._elapsed }
    pub fn add_channel(&mut self, cv : Vec<Box<Channel>>) {
        self._channels = cv;
    }
    pub fn schedule(&mut self, p : &mut Process, period : std::time::Duration) {
        Process::set_period(p, period);
    }
    pub fn drop(&mut self, p : &mut Process) {}
    pub fn init(&mut self) {
        for i in 0..self._processes.len() {
            self._processes[i]._init();
        }
        let starter = SystemTime::now();
        self._start_time = starter.duration_since(UNIX_EPOCH).expect("Time went backwards");
	    self._elapsed = starter.duration_since(UNIX_EPOCH).expect("Time went backwards") - self._start_time;
    }
    pub fn start(&mut self) {
        for i in 0..self._processes.len() {
            self._processes[i]._start(self._elapsed);
        }
    }
    pub fn stop(&mut self) {
        for i in 0..self._processes.len() {
            self._processes[i]._stop();
        }
    }
    pub fn run(&mut self, runtime : std::time::Duration) {
        println!("manager starting");
        self.start();
	    while self._elapsed < runtime {
            self.update();
            let temp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            self._elapsed = temp - self._start_time;
        }
        self.stop();
        println!("manager stopped");
    }
    pub fn update(&mut self) {
        for i in 0..self._processes.len() {
            if self._elapsed > self._processes[i].last_update() + self._processes[i].period() {
                self._processes[i]._update(&mut self._channels, self._elapsed);
            };
        }
    }
    pub fn channel(&mut self, name : String)->usize {
        for i in 0..self._channels.len() {
            if name == self._channels[i].name().to_string() {
                return i
            }
        }
        return 0
    }
    pub fn ps(&self)->HashMap<String, (String, u64, u64, i64)> {
        let mut info : HashMap<String, (String, u64, u64, i64)> = HashMap::new();

        for i in 0..self._processes.len() {
            let temp = ( self._processes[i].status_type_map(),
                        self._processes[i].milli_time(),
                        self._processes[i].delta(),
                        self._processes[i].num_updates()
            );
            info.insert( self._processes[i].name().to_string(), temp );
        }
        return info;
    }
}
