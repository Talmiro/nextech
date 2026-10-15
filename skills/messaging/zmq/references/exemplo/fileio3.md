# File Transfer with Credit-Based Flow Control

File transfer using DEALER/ROUTER with pipelining and credit-based flow control.

```rust
// examples/zguide/fileio3/main.rs
static CHUNK_SIZE: usize = 250_000;
static PIPELINE: usize = 10;
static PIPELINE_HWM: usize = 20;

fn client_thread(expected_total: usize) {
    let context = zmq::Context::new();
    let dealer = context.socket(zmq::DEALER).unwrap();
    let identity: Vec<u8> = (0..10).map(|_| rand::random::<u8>()).collect();
    dealer.set_identity(&identity).unwrap();
    dealer.connect("tcp://localhost:6000").unwrap();

    let mut credit = PIPELINE;
    let mut total = 0;
    let mut offset = 0;
    let mut clean_break = false;

    loop {
        while (credit > 0) && !clean_break {
            dealer.send("fetch", zmq::SNDMORE).unwrap();
            dealer.send(&offset.to_string(), zmq::SNDMORE).unwrap();
            dealer.send("250000", 0).unwrap();
            offset += CHUNK_SIZE;
            credit -= 1;
        }

        let chunk = dealer.recv_string(0).unwrap().unwrap();
        if chunk.is_empty() { clean_break = true; }
        credit += 1;
        total += chunk.len();
        if chunk.len() < CHUNK_SIZE { clean_break = true; }
        if clean_break && (credit == PIPELINE) { break; }
    }
    println!("{} chunks received, {} bytes", total / CHUNK_SIZE, total);
}

fn server_thread(file: &mut std::fs::File) -> std::io::Result<()> {
    let context = zmq::Context::new();
    let router = context.socket(zmq::ROUTER).unwrap();
    router.set_sndhwm(PIPELINE_HWM as i32).unwrap();
    router.bind("tcp://*:6000").unwrap();

    loop {
        let identity = router.recv_bytes(0).unwrap();
        if identity.is_empty() { break; }

        let _command = router.recv_string(0).unwrap().unwrap();
        let offset = router.recv_string(0).unwrap().unwrap().parse::<u64>().unwrap();
        let chunk_size = router.recv_string(0).unwrap().unwrap().parse::<usize>().unwrap();

        use std::io::Seek;
        file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        let mut data = vec![0; chunk_size];
        let size = std::io::Read::read(file, &mut data).unwrap();
        data.truncate(size);

        router.send(&identity, zmq::SNDMORE).unwrap();
        router.send(&data, 0).unwrap();
    }
    Ok(())
}
```

## Key Points

- Client requests chunks with credit-based flow control
- Server uses HWM to limit in-flight messages
- Chunk-based file transfer with random access
- Uses DEALER/ROUTER for async communication
