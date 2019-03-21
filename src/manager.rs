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
                let ind = self.channel("Data".to_string());
                //this works great, but in the original implementation we would pass the ownership of the channels vector
                //and channel function to the process, which would operate and then send it back. This doesn'r quite fit with
                //rust, so I am mulling it over.
                self._processes[i]._update(&mut self._channels[ind], self._elapsed);
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

        let f1 = |p : &Process| Process::status_type_map(p);
        let f2 = |p : &Process| Process::milli_time(p);
        let f3 = |p : &Process| Process::delta(p);
        let f4 = |p : &Process| Process::num_updates(p);
        let f5 = |p : &Process| Process::name(p).to_string();

        for i in 0..self._processes.len() {
            //let temp = ( f1(self._processes[i]), f2(self._processes[i]), f3(self._processes[i]), f4(self._processes[i]));
            //info.insert( f5(self._processes[i]), temp );
        }
        return info;
    }
}
