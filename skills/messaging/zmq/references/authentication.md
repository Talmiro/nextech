# Authentication Tests

CURVE, GSSAPI, and PLAIN authentication.

## CURVE Encryption

```rust
// tests/curve.rs - requires libsodium
let keypair = CurveKeyPair::new().unwrap();
assert!(keypair.public_key.len() == 32);

// Server setup
receiver.set_curve_server(true).unwrap();
receiver.set_curve_secretkey(&server_pair.secret_key).unwrap();

// Client setup  
sender.set_curve_serverkey(&server_pair.public_key).unwrap();
sender.set_curve_publickey(&client_pair.public_key).unwrap();
sender.set_curve_secretkey(&client_pair.secret_key).unwrap();

// Send encrypted message
sender.send("secret", 0).unwrap();
let msg = receiver.recv_msg(0).unwrap();
```

## GSSAPI (Kerberos)

```rust
// tests/gssapi.rs
sock.set_gssapi_server(true).unwrap();
sock.set_gssapi_principal("service@REALM").unwrap();
sock.set_gssapi_service_principal("kerberos").unwrap();
sock.set_gssapi_plaintext(true).unwrap();
```

## PLAIN Authentication

```rust
// tests/test.rs
sock.set_plain_server(true).unwrap();
sock.set_plain_username(Some("user")).unwrap();
sock.set_plain_password(Some("pass")).unwrap();
```

## Running

```bash
cd rust-zmq && cargo test curve    # CURVE tests
cd rust-zmq && cargo test gssapi   # GSSAPI tests
cd rust-zmq && cargo test plain    # PLAIN tests
```
