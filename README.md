# AppMem

AppMem is a local-first memory runtime for applications.

Instead of running a separate Redis server or external cache, applications connect to a lightweight local daemon that owns memory outside the application process.

This allows in-memory state to survive:

- application restarts
- deployments
- crashes
- hot reloads

while remaining extremely fast because communication happens over a local Unix socket instead of a network hop.

---

# Philosophy

AppMem is intentionally scoped differently from Redis.

The goal is not:

- distributed infrastructure
- massive clustering
- enterprise-scale persistence
- cloud-first architecture

The goal is:

> fast local application memory with minimal operational overhead

Start simple.

Scale later only if necessary.

---

# Positioning

> “Start without Redis. Scale when you actually need to.”

AppMem targets:

- indie hackers
- solo developers
- side projects
- small VPS deployments
- single-machine applications
- local-first infrastructure

---

# Core Features

## Planned Features

- ultra-fast local memory access
- survives application restarts
- TTL and expiration
- eviction policies
- namespaces
- temporary state management
- counters
- lightweight locks
- export/import tooling
- migration path to Redis later

---

# Example Vision

Instead of this:

```txt
App -> Redis TCP -> Redis Server
```

AppMem aims for:

```txt
App -> Unix Socket -> Local Memory Runtime
```

No external cache server required.

No network hop.

No Redis setup for small projects.

---

# Intended Architecture

## Components

```txt
appmemd        # memory daemon
appmem-cli     # CLI tool
appmem-node    # Node.js client
```

Applications communicate with the daemon over:

- Unix domain sockets
- local IPC

The daemon owns memory independently from the application process.

---

# Example Usage

## Node.js

```ts
import { AppMem } from "appmem";

const mem = new AppMem();

await mem.set("user:1", "Shanil");

const user = await mem.get("user:1");
```

---

# Current Status

🚧 Early development / learning project

This project is also a systems programming learning journey in Rust.

The goal is to gradually build toward:

- daemon architecture
- IPC
- concurrency
- memory management
- eviction systems
- protocol design
- lightweight infrastructure tooling

---

# Roadmap

## Phase 1

- in-memory HashMap store
- CLI interface
- TTL support
- namespaces

## Phase 2

- daemon process
- local socket communication
- concurrent clients
- JSON protocol

## Phase 3

- Node.js bindings
- framework adapters
- export/import
- eviction policies

## Phase 4

- optional persistence
- replication experiments
- distributed runtime exploration

---

# Why Build This?

Modern applications often pull in Redis extremely early even when:

- deployments are single-machine
- data is temporary
- persistence is unnecessary
- network hops add avoidable complexity

AppMem explores a different approach:

- local-first memory
- process-independent state
- lightweight operational model
- infrastructure that scales with actual needs

---

# Technical Goals

This project exists partly to learn:

- Rust
- ownership & borrowing
- concurrency
- Unix sockets
- daemon lifecycle management
- IPC
- serialization
- memory management
- systems programming

The compiler pain is part of the process.

---

# Non-Goals (for now)

AppMem is NOT trying to become:

- a full database
- a Redis replacement
- a distributed cache cluster
- a durable transactional system

At least not initially.

The focus is:

> single-machine application memory done well

---

# Long-Term Vision

Long-term, AppMem could evolve into:

- distributed memory runtimes
- cross-node memory replication
- shared application state
- lightweight orchestration primitives

But the immediate goal is much smaller:

Build something useful.
Learn deeply.
Understand systems programming properly.

---

# Inspiration

AppMem is inspired by the idea that:

small applications should not need enterprise infrastructure on day one.

A lot of software complexity is adopted too early.

This project explores what happens when infrastructure starts local-first instead.
