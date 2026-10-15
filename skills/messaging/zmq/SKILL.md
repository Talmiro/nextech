---
name: zmq
description: >
  ZeroMQ (zmq) - high-performance async messaging. PUB/SUB, REQ/REP, DEALER/ROUTER,
  PUSH/PULL, PAIR, STREAM, XPUB/XSUB patterns.

argument-hint: "[what-to-build]"
allowed-tools:
  - Read
  - Glob
  - Grep
  - Edit
  - Write
  - Bash
---

## Patterns (raiz)

- **pubsub.md** — Publisher-Subscriber
- **xpub.md** — Extended PUB/SUB
- **reqrep.md** — Request-Reply
- **dealerouter.md** — Async Request-Reply
- **pushpull.md** — Pipeline
- **pair.md** — Bidirectional pipe
- **stream.md** — Raw TCP

## Examples (exemplo/)

Basic: basic, reqrep, pubsub, pipeline, async, stream, benchmark

Advanced: fileio3, lbbroker, lvcache, msgqueue, mspoller, msreader, mtrelay, pathopub, pathosub, rtdealer, rtreq, syncpub, syncsub, tasksink2, taskwork2, wuproxy

## Tests

- **basictests.md** — Context, messages, errors, Z85
- **authentication.md** — CURVE, GSSAPI, PLAIN
- **sockets.md** — Send/receive, multipart, polling
- **options.md** — Socket options
- **transports.md** — inproc, TCP, IPC
- **advanced.md** — Monitor, poll, external loops
