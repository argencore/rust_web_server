use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

/// a struct to hold the worker threads and message sending channel
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// represents either a new job as defined below or a termination signal
enum Message{
    NewJob(Job),
    Terminate,
}

/// a job type defined as follows
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool{
    /**
        # Summary
        Create a new ThreadPool.

        # Arguments
        `size` - the number of threads in the pool

        # Panics
        The `new` function will panic if the size is zero.
    */
    pub fn new(size: usize)->ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
           workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender,
        }
    }

    /**
        # Summary
        sends a message with a new job to execute

        # Arguments
        `self` - threadpool
        `f` - where F is a job to execute

        # Panics
        if it cannot unwrap the new message
    */
    pub fn execute<F>(&self,f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    /**
        # Summary
        implements the drop method for the ThreadPool

        # Arguments
        `self` - ThreadPool as a mutable reference
    */
    fn drop(&mut self){
        println!("sending terminate message to all workers");

        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers{
            println!("shutting down worker: {}", worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

/// create a trait FnBox and give it the function signature call_box
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    /// implement call_box for F where F has the trait FnOnce()
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


//worker thread struct managed by the ThreadPool
struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    /// function to create a new worker, the mpsc::Receiver is wrapped in a mutex and reference counted pointer so that
    /// it can be modified by multiple threads which can be dangerous if not managed properly.
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread = thread::spawn(move||{
            loop{
                let message = reciever.lock().unwrap().recv().unwrap();
                match message{
                    Message::NewJob(job) =>{
                        println!("worker {} has a job running", id);
                        job.call_box();
                    }
                    Message::Terminate =>{
                        println!("worker {} was told to terminate", id);
                        break;
                    }
                }
            }
        });
        Worker{
            id,
            thread: Some(thread),
        }
    }
}

