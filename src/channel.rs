use std::collections::VecDeque;
use std::vec::Vec;

/// # Arguments
/// 
/// * `_name` - A string holding the name of the channel instantiation.
/// * `_capacity` - This contains the total capacity of the channel.
/// * `_queue` - A queue where all data is placed in and read out from the channel. 
/// 
/// # Remarks
/// 
/// All ELMA channels are meant to have the same fields. For this reason, all Channel fields are described in
/// the Channel struct.
#[allow(dead_code)]
pub struct Channel {
    pub _name : String,
    pub _capacity : usize,
    pub _queue : VecDeque<f64>,
}

/// # Remarks
/// 
/// The implementation of Channel shares fields from the pub struct Channel. This means that when a channel object
/// is created the object contains fields from the struct and one may operate on the object using the impl functions.
/// The given functions allow for full read/write access to the _capacity and _queue, but the _name is read only.
#[allow(dead_code)]
impl Channel {
    pub fn size(&self)->usize { self._queue.len() }
    pub fn empty(&self)->bool { self._queue.len() == 0 }    
    pub fn nonempty(&self)->bool { self._queue.len() > 0 }
    pub fn name(&self)->&String { &self._name }
    pub fn capacity(&self)->&usize { &self._capacity }

    pub fn send(&mut self, value:f64) {
        self._queue.push_front(value);
        //implement using capacity
        while self._queue.len() >= self._capacity {
            self._queue.pop_back();
        }
    }
    pub fn flush(&mut self) { self._queue.clear(); }
    //use preexisting size fn
    pub fn change_capacity(&mut self, new_cap : usize) {
        while self._queue.len() > new_cap {
            self._queue.pop_back();
        }
        self._capacity = new_cap;
    }
    //would like to figure out polymorphism in rust
    //use preexisting size fn
    pub fn latest_db(&self)->f64 {
        match self._queue.get(0) {
            Some(x)=> { *x },
            None=> { return 0.0 },
        }
    }
    //use preexisting size
    pub fn latest_vec(&self, n:usize)->Vec<f64> {
        let mut temp : Vec<f64> = Vec::new();
        for i in 0..n {
            match self._queue.get(i) {
                Some(x)=> { temp.push(*x); },
                None=> { return temp },
            }
        }
        temp
    }
    pub fn earliest(&self)->f64 {
        match self._queue.back() {
            Some(x)=> { *x },
            None=> { return 0.0 }
        }
    }
}
