# RelayMoQ

A distributed, low-latency pub/sub and relay system built on
QUIC and Media over QUIC (MoQ).

The project explores multi-region relay networks, efficient
data distribution, and eventually cloud gaming workloads.

## Vision

Build a distributed real-time data system where publishers can
send data through a network of relays to subscribers.

Video is not the core abstraction. It is one type of data that
the system should eventually support efficiently.

The long-term goal is to build a cloud-gaming architecture and
evaluate its latency, scalability, and multi-region behavior.

## Architecture

```text
                           Publisher
                               │
                               ▼
                    ┌───────────────────┐
                    │   Origin Relay    │
                    │      Austin       │
                    └─────────┬─────────┘
                              │
                 ┌────────────┴────────────┐
                 │                         │
                 ▼                         ▼
        ┌──────────────────┐      ┌──────────────────┐
        │    Edge Relay    │      │    Edge Relay    │
        │    New York      │      │    California    │
        └────────┬─────────┘      └────────┬─────────┘
                 │                         │
                 ▼                         ▼
          Subscribers               Subscribers
           (New York)               (California)
```

## Roadmap

### Milestone 1: QUIC Foundation

- [x] Create QUIC server
- [x] Create QUIC client
- [x] Establish connection
- [x] Open bidirectional streams
- [x] Send data
- [x] Receive data

### Milestone 2: Stream Communication

- [x] Echo messages
- [x] Support multiple messages
- [x] Keep a connection alive
- [x] Explore stream lifecycle

### Milestone 3: Application Protocol

#### 3.1 Message Framing

- [x] Identify message-boundary problem
- [x] Length-prefixed framing
- [x] Define byte ordering
- [x] Encode message length
- [x] Write framed messages
- [x] Read framed messages
- [x] Handle partial reads
- [x] Handle multiple messages
- [x] Handle empty messages
- [x] Reject oversized messages
- [x] Handle truncated headers
- [x] Handle truncated payloads
- [x] Add unit tests

## 3.2: Message Layer

- [x] Define Message enum
- [x] Define message types
  - [x] PUBLISH
  - [x] SUBSCRIBE
  - [x] UNSUBSCRIBE
  - [x] DATA
- [x] Define message structure
- [x] Define wire representation
- [x] Implement message encoding
- [x] Implement message decoding
- [x] Validate unknown message types
- [x] Validate malformed messages
- [x] Validate oversized topics
- [x] Validate oversized payloads
- [x] Unit test message encoding
- [x] Unit test message decoding
- [x] Test all message types
- [x] Send messages over QUIC
- [x] Decode received messages
- [x] Echo decoded messages

### Milestone 4: Generic Pub/Sub

- [x] Define topics
- [x] Implement SUBSCRIBE
- [x] Implement UNSUBSCRIBE
- [x] Implement PUBLISH
- [ ] Maintain subscriptions
- [ ] Fan out messages
- [x] Test multiple publishers (multiple publishers should not be allowed)
- [x] Test multiple subscribers

### Milestone 5: MoQ Integration

- [ ] Map pub/sub concepts to MoQ
- [ ] Define tracks
- [ ] Define groups
- [ ] Define objects
- [ ] Implement publisher
- [ ] Implement subscriber

### Milestone 6: Relay

- [ ] Build standalone relay
- [ ] Forward subscriptions
- [ ] Forward published data
- [ ] Support multiple publishers
- [ ] Support multiple subscribers
- [ ] Measure relay overhead

### Milestone 7: Multi-Relay Network

- [ ] Connect relays
- [ ] Relay-to-relay subscriptions
- [ ] Forward data across relays
- [ ] Implement fanout
- [ ] Build Austin/NYC/London topology
- [ ] Measure multi-hop latency
- [ ] Test relay failures

### Milestone 8: Video

- [ ] Integrate AV1 encoder
- [ ] Represent video as application data
- [ ] Publish video
- [ ] Subscribe to video
- [ ] Decode video
- [ ] Measure end-to-end latency
- [ ] Measure throughput
- [ ] Measure frame loss

### Milestone 9: Cloud Gaming

- [ ] Game input channel
- [ ] Game video channel
- [ ] Audio channel
- [ ] GPU-backed game instance
- [ ] Input → game latency measurement
- [ ] Game → display latency measurement
- [ ] Glass-to-glass latency
- [ ] Multi-region game sessions
- [ ] Compare against SFU architecture

## Design Principles

- QUIC provides transport.
- Framing provides message boundaries.
- Messages provide application semantics.
- Pub/sub provides data distribution.
- MoQ provides media-oriented transport semantics.
- Relays provide distribution across regions.
- Video is an application of the system, not the system itself.

## Goals

- Low latency
- Efficient fanout
- Multi-region distribution
- Minimal unnecessary data copying
- Observable networking behavior
- Understandable protocol layers
- Rust-first implementation
