use std::sync::{Arc, RwLock};
use std::thread;


#[derive(Clone)]
struct SafeCounter(Arc<RwLock<u32>>);

impl SafeCounter {
    pub fn new(val : u32) -> Self {
        SafeCounter(Arc::new(RwLock::new(val)))
    }

    pub fn next(&self) -> u32 {
        let mut counter = self.0.write().unwrap();
        *counter = *counter + 1;
        return *counter;
    }

    pub fn current(&self) -> u32 {
        *self.0.read().unwrap()
        
    }
}

fn main() {
    let counter = SafeCounter::new(0);
    let mut handles  = Vec::new();
    for _ in 1..4 {
        let cnt = counter.clone();
        let hdl = thread::spawn(move || {
            for _ in 0..10 {
                if cnt.current() < 10 {
                    let timeout: u32 = 1 + 10 * cnt.next();
                    thread::sleep_ms(timeout); 
                }
            }
        } );
        handles.push(hdl);
    }
   
    while let Some(handle) = handles.pop()
    {
        handle.join().unwrap();
    }
    println!("SafeCounter.current() => {}", counter.current());
}
