use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    // with_arc();
    with_channels();
}

pub fn with_arc() {
    let m = Arc::new(Mutex::new(String::from("moving")));
    let m2 = m.clone();
    std::thread::spawn(move || {
        println!("This is the new thread");
        let mut s = m2.lock().unwrap();
        s.push_str("on the new thread");
        println!("m = {}", s);
    });

    std::thread::sleep(Duration::from_millis(1000));
    println!("This is the initial thread");
    let s = m.lock().unwrap();
    println!("now m = {}", s);
}

fn with_channels() {
    let (ch_s, ch_r) = std::sync::mpsc::channel::<Box<Fn(&mut String) + Send>>();

    let (done_s, done_r) = std::sync::mpsc::channel::<()>();

    std::thread::spawn(move || {
        let mut hidden = String::new();
        loop {
            match ch_r.recv() {
                Ok(f) => {
                    f(&mut hidden);
                    println!("hidden  = {}", hidden);
                }
                Err(_) => {
                    println!("Done");
                    done_s.send(()).unwrap();
                    return;
                }
            }
        }
    });

    ch_s.send(Box::new(|s: &mut String| {
        s.push_str("Hello");
    }))
    .unwrap();

    let ch_2 = ch_s.clone();

    ch_2.send(Box::new(|s: &mut String| {
        s.push_str(" world");
    }))
    .unwrap();

    drop(ch_s);
    drop(ch_2);

    done_r.recv().ok();
    //std::thread::sleep(Duration::from_millis(1000));
}
