use super::scene::Scene;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Coordinate = (usize, usize, usize);
type Job = Box<dyn Fn(&Scene, (usize, usize, usize)) -> (u8, u8, u8) + Send + 'static>;

pub enum Message {
    NewJob(Coordinate, Job),
    Terminate,
}

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/* Thread pool to distribute the calculations over threads */

impl ThreadPool {
    pub fn new(size: usize, scene: Scene) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), scene.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn schedule<F>(&self, coordinate: Coordinate, f: F)
    where
        F: Fn(&Scene, Coordinate) -> (u8, u8, u8) + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(coordinate, job)).unwrap();
    }

    pub fn wait_all(&mut self) {
        println!("Send Terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    pub id: usize,
    pub handle: Option<thread::JoinHandle<()>>,
    pub results: Arc<Mutex<HashMap<usize, (u8, u8, u8)>>>, // index -> (r, g, b)
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        arc_scene: Scene,
    ) -> Worker {
        // Create references
        let results_map = Arc::new(Mutex::new(HashMap::new()));
        let scene = arc_scene.clone();
        let map = Arc::clone(&results_map);
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(coordinate, job) => {
                    let (r, g, b) = job(&scene, (coordinate.0, coordinate.1, coordinate.2));

                    // Add result to map.
                    map.lock().unwrap().insert(coordinate.2, (r, g, b));
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(thread),
            results: Arc::clone(&results_map),
        }
    }
}
