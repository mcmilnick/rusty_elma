use std::vec::Vec;
use std::collections::HashMap;
use process::Process;
use channel::Channel;
use std::boxed;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[allow(dead_code)]
pub struct Manager {
    pub _processes : Vec<Box<Process>>,
    pub _channels : Vec<Box<Channel>>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

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
            for i in 0..self._processes.len() {
                if self._elapsed > self._processes[i].last_update() + self._processes[i].period() {
                    //let dataChan = self._channels
                    //self._processes[i]._update(data, self._elapsed);
                };
            }
            let temp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            self._elapsed = temp - self._start_time;
        }
        self.stop();
        println!("manager stopped");
    }
    /*pub fn ps(&self)->HashMap<String, (String, u64, u64, i64)> {
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
    }*/
}

/*
        Manager& all(std::function<void(Process&)> f);
        map<string, tuple<string, double, double, int>> ps();
        Manager& update();
*/