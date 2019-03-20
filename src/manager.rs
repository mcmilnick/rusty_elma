extern crate colored;

use std::collections::VecDeque;
use std::vec::Vec;
use std::collections::HashMap;
use process::Process;
use process;
use channel::Channel;

//a large portion of the running pieces need to be pulled out of manager because it involves cyclic data ownership of processes and manager
pub struct Manager<'a> {
    pub _processes : Vec<&'a Process>,
    pub _channels : HashMap<String, &'a mut Channel>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

impl<'a> Manager<'a> {
    pub fn start_time(&self)->std::time::Duration { self._start_time }
    pub fn elapsed(&self)->std::time::Duration { self._elapsed }
    pub fn add_channel(&mut self, c : &'a mut Channel) {
        self._channels.insert(c.name().to_string(), c);
    }
    pub fn schedule(&mut self, p : &'a mut Process, period : std::time::Duration) {
        p._period = period;
        self._processes.push(p);
    }
    pub fn drop(p : &'a Process) {}
    pub fn ps(&self)->HashMap<String, (String, u64, u64, i64)> {
        let mut info : HashMap<String, (String, u64, u64, i64)> = HashMap::new();

        let f1 = |p : &Process| process::Process::status_type_map(p);
        let f2 = |p : &Process| process::Process::milli_time(p);
        let f3 = |p : &Process| process::Process::delta(p);
        let f4 = |p : &Process| process::Process::num_updates(p);
        let f5 = |p : &Process| process::Process::name(p).to_string();

        for i in 0..self._processes.len() {
            match self._processes.get(i) {
                Some(proc_ptr)=> {
                    let temp = ( f1(proc_ptr), f2(proc_ptr), f3(proc_ptr), f4(proc_ptr));
                    info.insert( f5(proc_ptr), temp );
                },
                None=> {},
            }
        }
        return info;
    }
}
