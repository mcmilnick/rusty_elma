use std::vec::Vec;
use std::collections::HashMap;
use process::Process;
use channel::Channel;
use std::boxed;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[allow(dead_code)]
pub struct Manager {
    pub _processes : Vec<String>,
    pub _channels : Vec<String>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

#[allow(dead_code)]
impl Manager {
    pub fn start_time(&self)->std::time::Duration { self._start_time }
    pub fn elapsed(&self)->std::time::Duration { self._elapsed }
    pub fn add_channel(&mut self, c : &mut Channel) {
        self._channels.push(c.name().to_string());
    }
    pub fn schedule(&mut self, p : &mut Process, period : std::time::Duration) {
        Process::set_period(p, period);
        self._processes.push(p.name().to_string());
    }
    pub fn drop(&mut self, p : &mut Process) {
        match self._processes.binary_search(&p.name().to_string()) {
            Ok(ind) => { self._processes.remove(ind); },
            _ => {},
        }
    }
    pub fn init(&mut self, vb : &mut Vec<Box<Process>>) {
        for i in 0..vb.len() {
            vb[i]._init();
        }
        let starter = SystemTime::now();
        self._start_time = starter.duration_since(UNIX_EPOCH).expect("Time went backwards");
	    self._elapsed = starter.duration_since(UNIX_EPOCH).expect("Time went backwards") - self._start_time;
    }
    pub fn start(&mut self, vb : &mut Vec<Box<Process>>) {
        for i in 0..vb.len() {
            vb[i]._start(self._elapsed);
        }
    }
    pub fn stop(&mut self, vb : &mut Vec<Box<Process>>) {
        for i in 0..vb.len() {
            vb[i]._stop();
        }
    }
    pub fn run(&mut self, vb : &mut Vec<Box<Process>>, data : &mut Vec<Box<Channel>>, run_time : u64) {
        let runtime = Duration::new(run_time, 0);
	    while self._elapsed < runtime {
            for i in 0..vb.len() {
                if self._elapsed > vb[i].last_update() + vb[i].period() {
                    vb[i]._update(&mut data[0], self._elapsed);
                };
            }
            let temp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            self._elapsed = temp - self._start_time;
        }

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