extern crate colored;

mod channel_mod {
    use std::collections::VecDeque;
    use std::vec::Vec;
    pub struct Channel {
        pub _name : String,
        pub _capacity : usize,
        pub _queue : VecDeque<f64>,
    }
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
        pub fn latest_db(&mut self)->f64 {
            match self._queue.get(0) {
                Some(x)=> { *x },
                None=> { return 0.0 },
            }
        }
        //use preexisting size
        pub fn latest_vec(&mut self, n:usize)->Vec<f64> {
            let mut temp : Vec<f64> = Vec::new();
            for i in 0..n {
                match self._queue.get(i) {
                    Some(x)=> { temp.push(*x); },
                    None=> { return temp },
                }
            }
            temp
        }
        pub fn earliest(&mut self)->f64 {
            match self._queue.back() {
                Some(x)=> { *x },
                None=> { return 0.0 }
            }
        }
    }
}
