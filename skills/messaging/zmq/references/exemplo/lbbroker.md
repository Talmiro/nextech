# Load Balancing Broker

LRU (Least Recently Used) load balancing between clients and workers using ROUTER/ROUTER.

```rust
// examples/zguide/lbbroker/main.rs
fn client_task(client_nbr: i32) {
    let context = zmq::Context::new();
    let client = context.socket(zmq::REQ).unwrap();
    let identity = format!("Client{}", client_nbr);
    client.set_identity(identity.as_bytes()).unwrap();
    client.connect("ipc://frontend.ipc").unwrap();

    client.send("HELLO", 0).unwrap();
    let reply = client.recv_string(0).unwrap().unwrap();
    println!("Client: {}", reply);
}

fn worker_task(worker_nbr: i32) {
    let context = zmq::Context::new();
    let worker = context.socket(zmq::REQ).unwrap();
    let identity = format!("Worker{}", worker_nbr);
    worker.set_identity(identity.as_bytes()).unwrap();
    worker.connect("ipc://backend.ipc").unwrap();

    worker.send("READY", 0).unwrap();
    loop {
        let identity = worker.recv_string(0).unwrap().unwrap();
        let _empty = worker.recv_string(0).unwrap().unwrap();
        let request = worker.recv_string(0).unwrap().unwrap();
        println!("Worker: {}", request);

        worker.send(&identity, zmq::SNDMORE).unwrap();
        worker.send("", zmq::SNDMORE).unwrap();
        worker.send("OK", 0).unwrap();
    }
}

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::ROUTER).unwrap();
    frontend.bind("ipc://frontend.ipc").unwrap();
    backend.bind("ipc://backend.ipc").unwrap();

    let mut worker_queue = Vec::new();
    loop {
        let mut items = [
            backend.as_poll_item(zmq::POLLIN),
            frontend.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items[0..if worker_queue.is_empty() { 1 } else { 2 }], -1).unwrap();

        if items[0].is_readable() {
            let worker_id = backend.recv_string(0).unwrap().unwrap();
            let _empty = backend.recv_string(0).unwrap().unwrap();
            worker_queue.push(worker_id);

            let client_id = backend.recv_string(0).unwrap().unwrap();
            if client_id != "READY" {
                let _empty = backend.recv_string(0).unwrap().unwrap();
                let reply = backend.recv_string(0).unwrap().unwrap();
                frontend.send(&client_id, zmq::SNDMORE).unwrap();
                frontend.send("", zmq::SNDMORE).unwrap();
                frontend.send(&reply, 0).unwrap();
            }
        }

        if items[1].is_readable() {
            let client_id = frontend.recv_string(0).unwrap().unwrap();
            let _empty = frontend.recv_string(0).unwrap().unwrap();
            let request = frontend.recv_string(0).unwrap().unwrap();

            let worker = worker_queue.pop().unwrap();
            backend.send(&worker, zmq::SNDMORE).unwrap();
            backend.send("", zmq::SNDMORE).unwrap();
            backend.send(client_id.as_bytes(), zmq::SNDMORE).unwrap();
            backend.send("", zmq::SNDMORE).unwrap();
            backend.send(&request, 0).unwrap();
        }
    }
}
```

## Key Points

- Uses ROUTER/ROUTER for both frontend and backend
- LRU queue: worker that finishes first gets next client request
- Workers send "READY" to signal availability
- Clients and workers use REQ sockets with identities
