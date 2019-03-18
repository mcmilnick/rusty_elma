extern crate colored;

use std::collections::VecDeque;
use std::vec::Vec;
use std::collections::HashMap;
use process::Process;
use process;
use channel::Channel;

pub struct Manager<'a> {
    pub _processes : Vec<&'a Process>,
    pub _channels : HashMap<String, &'a Channel>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

impl<'a> Manager<'a> {
    pub fn start_time(&self)->std::time::Duration { self._start_time }
    pub fn elapsed(&self)->std::time::Duration { self._elapsed }
    pub fn add_channel(&mut self, c : &'a Channel) {
        self._channels.insert(c.name().to_string(), c);
    }
    pub fn schedule(&mut self, p : &'a mut Process, period : std::time::Duration) {
        p._period = period;
        self._processes.push(p);
        //this should be a manager pointer, but to pass a reference to self is a terrible idea in rust
        //come back to solve - only effects channel call in process
        //p._manager_ptr = self;
    }
    pub fn channel(&self, s : String)->&'a Channel {
        match self._channels.get(&s) {
            Some(aChannel) => { return aChannel },
            None => { panic!("no channel"); },
        }
    }
    pub fn drop(p : &'a Process) {}
    pub fn all(&self, f : &dyn Fn(&Process)) {
        for i in 0..self._processes.len() {
            match self._processes.get(i) {
                Some(proc_ptr)=> { f(proc_ptr); },
                None=> {},
            }
        }
    }
    pub fn init(&self) {
        let myfunc = |p : &Process| process::Process::_init();
        Manager::all(&self, &myfunc);
    }
    pub fn start(&self) {
        let myfunc = |p : &Process| process::Process::_start(self._elapsed);
        Manager::all(&self, &myfunc);
    }
    pub fn stop(&self) {
        let myfunc = |p : &Process| process::Process::_stop();
        Manager::all(&self, &myfunc);
    }
    pub fn update(&self) {
        let myfunc = |p : &Process|
            if self._elapsed > process::Process::last_update(&p) + process::Process::period(&p) {
                process::Process::_update(self._elapsed);
            };
        Manager::all(&self, &myfunc);
    }
    pub fn run(&mut self, runtime : std::time::Duration) {
        let starter = std::time::SystemTime::now();
        self._start_time = starter.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        Manager::start(&self);

        while self._elapsed < runtime {
            Manager::update(&self);
            let temp = std::time::SystemTime::now();
            self._elapsed = temp.duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                - self._start_time;
        }

        Manager::stop(&self);
    }
    pub fn ps(&self)->HashMap<String, (String, u64, f64, i64)> {
        let mut info : HashMap<String, (String, u64, f64, i64)> = HashMap::new();

        let f1 = |p : &Process| process::Process::status_type_map(p);
        let f2 = |p : &Process| process::Process::milli_time(p);
        let f3 = |p : &Process| process::Process::delta();
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
