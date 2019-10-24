use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub struct ThreadPool {
    ch_s: Option<mpsc::Sender<Box<Fn() + Send>>>,
    n: u32,
    ch_done: mpsc::Receiver<()>,
}

impl ThreadPool {
    pub fn new(n: u32) -> Self {
        let (ch_s, ch_r) = mpsc::channel();
        let a = Arc::new(Mutex::new(ch_r));

        let (ch_done_s, ch_done) = mpsc::channel();

        for _ in 0..n {
            let a2 = a.clone();
            let ch_done_2 = ch_done_s.clone();
            std::thread::spawn(move || loop {
                let m = a2.lock().unwrap();
                let f: Box<Fn() + Send> = match m.recv() {
                    Ok(f) => f,
                    Err(_) => {
                        ch_done_2.send(()).ok();
                        return;
                    }
                };
                drop(m);
                f();
            });
        }

        ThreadPool {
            ch_s: Some(ch_s),
            n,
            ch_done,
        }
    }

    pub fn run<F: Fn() + Send + 'static>(&self, f: F) {
        if let Some(ref ch_s) = self.ch_s {
            ch_s.send(Box::new(f)).unwrap();
        }
    }

    pub fn wait(mut self) {
        self.ch_s.take();
        for _ in 0..self.n {
            self.ch_done.recv().unwrap();
        }
    }
}

fn main() {
    let tp = ThreadPool::new(5);
    for i in 0..100 {
        tp.run(move || {
            std::thread::sleep(Duration::from_millis(200));
            println!("run = {}", i);
        })
    }

    tp.wait();

    //std::thread::sleep(Duration::from_millis(3000));
}
