use std::thread;
use callback::Runtime;
thread_local! {
    static RT: Runtime = Runtime::new();
}

fn main() {
    RT.with(|rt| rt.run(program));
}

fn program() {
    println!("Start the program here!");
    set_timeout(300, || {
        println!("Set a callback with 300ms.");
    });
    set_timeout(400, || {
        println!("Set a callback with 400ms");
        set_timeout(100, || {
            println!("Set a chain sub-callback.");
        })
    });
    println!("Waiting...");
}

fn set_timeout(ms: u64, cb: impl FnOnce() + 'static) {
    RT.with(|rt| {
        let id = rt.set_cb(cb).unwrap();
        let sender = rt.event_sender();
        thread::spawn(move || {
           thread::sleep(std::time::Duration::from_millis(ms));
            sender.send(id).unwrap();
        });
    })
}

