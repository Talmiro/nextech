# Asynchronous Server Pattern (DEALER/ROUTER)

The DEALER/ROUTER pattern provides asynchronous request-response communication.

## Asynchronous Server

```rust
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::{str, thread};

fn client_task() {
    let context = zmq::Context::new();
    let client = context.socket(zmq::DEALER).unwrap();
    let mut rng = thread_rng();
    let identity = format!("{:04X}-{:04X}", rng.gen::<u16>(), rng.gen::<u16>());
    client.set_identity(identity.as_bytes()).expect("failed setting client id");
    client.connect("tcp://localhost:5570").expect("failed connecting client");

    let mut request_nbr = 0;
    loop {
        for _ in 0..100 {
            if client.poll(zmq::POLLIN, 10).expect("client failed polling") > 0 {
                let msg = client.recv_multipart(0).expect("client failed receiving");
                println!("{}", str::from_utf8(&msg[msg.len() - 1]).unwrap());
            }
        }
        request_nbr += 1;
        let request = format!("request #{}", request_nbr);
        client.send(&request, 0).expect("client failed sending");
    }
}

fn server_task() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    frontend.bind("tcp://*:5570").expect("server failed binding frontend");
    let backend = context.socket(zmq::DEALER).unwrap();
    backend.bind("inproc://backend").expect("server failed binding backend");

    for _ in 0..5 {
        let ctx = context.clone();
        thread::spawn(move || server_worker(&ctx));
    }
    zmq::proxy(&frontend, &backend).expect("server failed proxying");
}

fn server_worker(context: &zmq::Context) {
    let worker = context.socket(zmq::DEALER).unwrap();
    worker.connect("inproc://backend").expect("worker failed");
    let mut rng = thread_rng();

    loop {
        let identity = worker.recv_string(0).unwrap().unwrap();
        let message = worker.recv_string(0).unwrap().unwrap();
        let replies = rng.gen_range(0..4);
        for _ in 0..replies {
            thread::sleep(Duration::from_millis(rng.gen_range(0..1000) + 1));
            worker.send(&identity, zmq::SNDMORE).expect("worker failed");
            worker.send(&message, 0).expect("worker failed");
        }
    }
}

fn main() {
    thread::spawn(client_task);
    thread::spawn(client_task);
    thread::spawn(client_task);
    thread::spawn(server_task);
    thread::sleep(Duration::from_secs(5));
}
```

## Multithreaded Server

```rust
use std::thread;
use std::time::Duration;

fn worker_routine(context: &zmq::Context) {
    let receiver = context.socket(zmq::REP).unwrap();
    receiver.connect("inproc://workers").expect("failed");
    loop {
        receiver.recv_string(0).expect("worker failed").unwrap();
        thread::sleep(Duration::from_millis(1000));
        receiver.send("World", 0).unwrap();
    }
}

fn main() {
    let context = zmq::Context::new();
    let clients = context.socket(zmq::ROUTER).unwrap();
    let workers = context.socket(zmq::DEALER).unwrap();

    clients.bind("tcp://*:5555").expect("failed");
    workers.bind("inproc://workers").expect("failed");

    for _ in 0..5 {
        let ctx = context.clone();
        thread::spawn(move || worker_routine(&ctx));
    }
    zmq::proxy(&clients, &workers).expect("failed");
}
```

## Architecture

```
Client (DEALER) <---> ROUTER (frontend) <---> DEALER (backend) <---> Workers
```

## Key Points

- ROUTER tracks client identity (socket must set identity)
- DEALER is asynchronous (can send/receive without blocking)
- Use `set_identity()` to identify clients
- Use `inproc://` for in-process communication between frontend/backend
- zmq::proxy connects ROUTER to DEALER automatically
- Workers can send multiple replies per request
