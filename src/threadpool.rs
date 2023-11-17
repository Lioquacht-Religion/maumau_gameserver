use std::{sync::{Arc, mpsc, Mutex},thread};
use thread::JoinHandle;


type Job =  Box< dyn FnOnce() + Send + Sync + 'static>;

struct Worker{
    id : usize,
    thread : Option<JoinHandle<()>>,
}

impl Worker{
    fn new(id : usize, reciever : Arc<Mutex<mpsc::Receiver<Job>>> ) -> Self{
        Self{
            id,
            thread : Some (thread::spawn(move || loop {
                let message = reciever.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job()
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            })),

        }
    }
}

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender : Option<mpsc::Sender<Job>>,
}


impl ThreadPool{

    pub fn new(size : usize) -> Self {
        let (sender, reciever) = mpsc::channel();
        let mut workers : Vec<Worker> = Vec::with_capacity(size);
        let reciever = Arc::new(Mutex::new(reciever));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&reciever)));
        }
        Self{
            workers,
            sender : Some(sender),
        }
    }

    pub fn execute<F>(&self, f : F)
    where
        F: FnOnce() + Send + Sync + 'static
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
