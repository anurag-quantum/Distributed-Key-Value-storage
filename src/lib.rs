use std::{sync::{mpsc,Arc,Mutex}, thread};


pub struct ThreadPool{
    threads : Vec<Worker>,
    sender :  mpsc::Sender<Job>,
}

pub struct Worker{
    id: i32,
    thread: thread::JoinHandle<()>,
}

impl Worker{
    pub fn new(id: i32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker{ id , thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/*
1. our thread pool should have an interface similar to thread::spawn. 
2. In addition, we’ll implement the execute function so it takes the closure it’s given and 
   gives it to an idle thread in the pool to run.
*/

impl ThreadPool {

        /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let mut threads = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size{
            // create threads = size and store it in the vector
            threads.push(Worker::new(i.try_into().unwrap() ,Arc::clone(&receiver)));

        }

        ThreadPool{ threads, sender }
    }

    pub fn execute<F>(&self, f:F )
    where F:FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        
    }
}

