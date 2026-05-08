use std::sync::{mpsc,Arc,Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{ workers: Vec<Worker>, sender: mpsc::Sender<Message> }

enum Message{ NewJob(Job), Terminate }

impl ThreadPool{
    pub fn new(size:usize)->Self{
        assert!(size>0);
        let (sender,receiver)=mpsc::channel::<Message>();
        let receiver=Arc::new(Mutex::new(receiver));
        let mut workers=Vec::with_capacity(size);
        for id in 0..size { workers.push(Worker::new(id, Arc::clone(&receiver))); }
        Self{workers,sender}
    }
    pub fn execute<F>(&self, f:F) where F:FnOnce()+Send+'static { self.sender.send(Message::NewJob(Box::new(f))).unwrap(); }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in &self.workers { let _ = self.sender.send(Message::Terminate); }
        for w in &mut self.workers { if let Some(t)=w.thread.take(){ let _=t.join(); } }
    }
}

struct Worker{ #[allow(dead_code)] id:usize, thread: Option<thread::JoinHandle<()>> }

impl Worker{
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Self{
        let thread=thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg { Message::NewJob(job)=>job(), Message::Terminate=>break }
        });
        Self{id, thread:Some(thread)}
    }
}

#[cfg(test)]
mod tests{ use super::*; use std::sync::atomic::{AtomicUsize,Ordering}; use std::sync::Arc;
    #[test] fn runs_jobs(){ let pool=ThreadPool::new(2); let c=Arc::new(AtomicUsize::new(0));
        for _ in 0..10 { let c2=c.clone(); pool.execute(move || { c2.fetch_add(1, Ordering::SeqCst); }); }
        drop(pool);
        assert_eq!(c.load(Ordering::SeqCst),10);
    }
}
