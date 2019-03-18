extern crate colored;

use std::collections::VecDeque;
use std::vec::Vec;
use std::collections::HashMap;
    
pub struct Manager {
    //should i do this with raw pointer
    pub _processes : Vec<&Process>,
    pub _channels : HashMap<String, &Channel>,
    pub _start_time : std::time::Duration,
	pub _elapsed : std::time::Duration,
}

impl Manager {
    pub fn start_time(&self)->std::time::Duration { self._start_time }
    pub fn elapsed(&self)->std::time::Duration { self._elapsed; }
    pub fn add_channel(c : &Channel)->&Manager {}
    pub fn schedule(p : &Process, period : std::time::Duration)->&Manager {}
    pub fn channel(s : String)->&Channel {}
    pub fn drop(p : &Process)->&Manager {}
    pub fn all(f : &Fn(&Process))->&Manager {}
    pub fn init()->&Manager {}
    pub fn start()->&Manager {}
    pub fn stop()->&Manager {}
    pub fn run(std::time::Duration)->&Manager {}
    pub fn ps()->HashMap<string, (string, double, double, int)> {}
    pub fn update()->&Manager {}
}
