use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::{ Mutex, Arc };

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      if let Some(thread) = worker.thread.take() {
        println!("Dropping worker with id: {}", worker.id);

        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
  pub fn new(threads: usize) -> Self {
    assert!(threads > 0);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let workers = (0..threads).map(|i| {
      Worker::new(i, Arc::clone(&receiver))
    }).collect();

    ThreadPool {
      workers,
      sender
    }
  }

  pub fn execute<F>(&self, f: F) where
    F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
    let thread = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      match job {
        Message::NewJob(new_job) => {
          println!("Worker {} got a job; executing.", id);

          new_job()
        },
        Message::Terminate => {
          println!("Worker {} will be stopped; terminating.", id);

          break;
        }
      }
    });

    Worker { id, thread: Some(thread) }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate
}
