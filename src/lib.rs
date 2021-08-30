use std::sync::mpsc;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
#[allow(bare_trait_objects)]
type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id ,Arc::clone(&receiver)));
        }
        ThreadPool { workers , sender} 
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce()+Send+'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

trait FnBox {
    fn call_box(self:Box<Self>);
}
impl<F:FnOnce()> FnBox for F {
    fn call_box(self : Box<F>){
        (*self)()
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down：{}",worker.id);
            if let Some(thread)= worker.thread.take() {
                let e = thread.join();
                match e {
                    Ok(_) => println!(""),
                    Err(_) => println!(""),
                };
            }
        }
    }
}

impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock();
            match job {
                Ok(job) =>{
                    println!("Worker {} executing",id);
                    let e = job.recv();
                    match e {
                        Ok(e) => {
                            e.call_box();
                        },
                        Err(_) => {
                            return
                        },
                    };
                },
                Err(_)=>{
                    return
                },
            };

            
        });
        Worker {id , thread:Some(thread) }
    }
}