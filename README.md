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

## Local Demo

Start the relay in one terminal:

```sh
cargo run --bin relay
```

Then run the pub/sub example in another terminal:

```sh
cargo run --example pubsub
```

The example opens separate subscriber and publisher connections for `game-123`, then prints the three DATA messages received by the subscriber. The relay currently permits one publisher per channel, so running two publishers for the same channel is rejected by design.

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
- [x] Explore stream lifecycle
- [x] Separate relay server from example clients

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

#### 3.2 Message Layer

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

### Milestone 4: Generic Pub/Sub

- [x] Define topics
- [x] Implement SUBSCRIBE
- [x] Implement UNSUBSCRIBE
- [x] Implement PUBLISH
- [x] Maintain subscriptions
- [x] Track channel publishers
- [x] Restrict DATA to the registered publisher
- [x] Fan out DATA to subscribers
- [x] Support multiple subscribers
- [x] Prevent multiple publishers on the same topic
- [x] Handle client disconnect cleanup
- [x] Add relay lifecycle tests

### Milestone 5: Relay Service

- [x] Build standalone relay
- [x] Separate protocol and relay layers
- [x] Introduce Relay service boundary
- [x] Track connected clients
- [x] Maintain per-client outbound channels
- [x] Forward published data to subscribers
- [x] Clean up client state on disconnect
- [ ] Implement QUIC outbound writer
- [ ] Add end-to-end publisher → relay → subscriber test
- [ ] Define backpressure behavior
- [ ] Define connection and resource limits

### Milestone 6: Cloud Gaming Example

- [ ] Create cloud gaming example
- [ ] Simulate game server as publisher
- [ ] Simulate game client as subscriber
- [ ] Publish game state/frame data
- [ ] Receive game data at the client
- [ ] Measure relay forwarding latency

### Milestone 7: MoQ Integration

- [ ] Map pub/sub concepts to MoQ
- [ ] Define tracks
- [ ] Define groups
- [ ] Define objects
- [ ] Implement MoQ publisher
- [ ] Implement MoQ subscriber
- [ ] Compare application-level framing with MoQ framing

### Milestone 8: Multi-Relay Network

- [ ] Connect relays
- [ ] Relay-to-relay subscriptions
- [ ] Forward data across relays
- [ ] Implement inter-relay fanout
- [ ] Build Austin / New York / London topology
- [ ] Measure multi-hop latency
- [ ] Test relay failures

### Milestone 9: Video

- [ ] Integrate AV1 encoder
- [ ] Represent video as application data
- [ ] Publish video
- [ ] Subscribe to video
- [ ] Decode video
- [ ] Measure end-to-end latency
- [ ] Measure throughput
- [ ] Measure frame loss

### Milestone 10: Cloud Gaming

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
