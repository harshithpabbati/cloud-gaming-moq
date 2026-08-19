# moq-rs

A from-scratch exploration of Media over QUIC (MoQ), built in Rust.

The goal is to understand and implement the networking primitives behind MoQ, build a generic pub/sub relay system, and eventually use video as an application on top of it.

The project is intentionally being built incrementally. Each milestone should result in something working and understandable before moving to the next one.

---

# Roadmap

## Phase 0: Project Foundation

- [x] Create Rust project
- [x] Set up `client` and `server` binaries
- [x] Set up QUIC with Quinn
- [x] Configure TLS
- [x] Configure Rustls crypto provider
- [x] Set up Rust formatting
- [x] Set up GStreamer
- [x] Verify camera access

---

# Phase 1: Media Experiments

These experiments are separate from the core MoQ implementation.

- [x] Capture live camera
- [x] Configure camera to 1280x720 @ 30 FPS
- [x] Capture raw video frames
- [x] Encode video using AV1
- [x] Encode video using H.264
- [x] Create codec-independent `VideoEncoder`
- [x] Create `EncodedVideoUnit`
- [x] Identify keyframes
- [x] Configure bitrate
- [x] Configure low-latency encoding
- [x] Configure keyframe intervals
- [x] Verify encoded output

### Future Media Work

- [ ] Add video decoder
- [ ] Display decoded video
- [ ] Capture microphone audio
- [ ] Encode audio
- [ ] Decode audio
- [ ] Synchronize audio and video
- [ ] Add codec selection

---

# Phase 2: QUIC Fundamentals

## Milestone 2.1: Basic Connection

- [x] Client creates QUIC endpoint
- [x] Server creates QUIC endpoint
- [x] Client connects to server
- [x] Server accepts connection
- [x] Verify connection lifecycle

## Milestone 2.2: Bidirectional Streams

- [x] Client opens bidirectional stream
- [x] Server accepts bidirectional stream
- [x] Client sends bytes
- [x] Server receives bytes
- [x] Server sends bytes
- [x] Client receives bytes
- [x] Understand `send` / `recv`
- [x] Understand `open_bi()` / `accept_bi()`
- [x] Understand stream completion with `finish()`
- [x] Understand connection lifetime

## Milestone 2.3: Echo Server

- [x] Client sends a message
- [x] Server receives the message
- [x] Server echoes the exact bytes back
- [x] Client receives the echoed message
- [x] Verify the echoed payload matches the original payload
- [x] Support multiple messages over a connection
- [x] Keep a stream open for multiple messages

---

# Phase 3: Application Message Layer

QUIC provides byte streams. It does not provide application-level message boundaries.

## Milestone 3.1: Message Framing

- [x] Identify the message-boundary problem
- [x] Design a simple framing format
- [x] Define byte ordering
- [x] Encode message length
- [x] Write framed messages
- [x] Read framed messages
- [x] Handle partial reads
- [x] Handle multiple messages in one stream
- [x] Handle malformed frames
- [x] Handle oversized messages

Example conceptual format:

    +----------------+-------------------+
    | message length | message payload   |
    +----------------+-------------------+

## Milestone 3.2: Generic Messages

- [ ] Define a generic message abstraction
- [ ] Define message types
- [ ] Serialize messages
- [ ] Deserialize messages
- [ ] Handle unknown message types
- [ ] Handle invalid messages
- [ ] Separate transport from message handling

Target architecture:

    Application
         |
         v
    Message Layer
         |
         v
    QUIC Stream
         |
         v
    QUIC Connection

---

# Phase 4: MoQ Data Model

Start introducing actual MoQ concepts.

## Tracks

- [ ] Understand Track
- [ ] Define Track identity
- [ ] Create a Track
- [ ] Remove a Track
- [ ] Track lifecycle

## Groups

- [ ] Understand Groups
- [ ] Define Group identity
- [ ] Associate Objects with Groups
- [ ] Maintain Group ordering

## Objects

- [ ] Understand Objects
- [ ] Define Object identity
- [ ] Define Object payload
- [ ] Define Object metadata
- [ ] Maintain Object ordering
- [ ] Publish Objects to a Track

Initial payload should remain simple:

    Track: "hello"

    Object 0: "hello"
    Object 1: "world"
    Object 2: "from moq-rs"

---

# Phase 5: Generic Pub/Sub

No video yet.

## Milestone 5.1: Publisher

- [ ] Create Publisher abstraction
- [ ] Create Track
- [ ] Publish Objects
- [ ] Publish multiple Objects
- [ ] Handle publisher disconnect

## Milestone 5.2: Subscriber

- [ ] Create Subscriber abstraction
- [ ] Request a Track
- [ ] Subscribe to Track
- [ ] Receive Objects
- [ ] Maintain Object ordering
- [ ] Unsubscribe

## Milestone 5.3: Single Relay

- [ ] Create Track registry
- [ ] Register published Tracks
- [ ] Accept subscriptions
- [ ] Forward Objects
- [ ] Handle unknown Tracks
- [ ] Handle Publisher disconnect
- [ ] Handle Subscriber disconnect

Target:

                 Publisher
                     |
                     v
                   Relay
                     |
                     v
                 Subscriber

## Milestone 5.4: Multiple Subscribers

- [ ] Support multiple subscribers
- [ ] Fan out Objects
- [ ] Maintain independent subscriber state
- [ ] Handle slow subscribers
- [ ] Handle subscriber disconnect
- [ ] Avoid unnecessary payload copies

Target:

                    Relay
                  /  |  \
                 /   |   \
                v    v    v
              Sub A Sub B Sub C

---

# Phase 6: Relay-to-Relay

Turn the single relay into a distributed relay system.

Target:

                 Publisher
                     |
                     v
                  Relay A
                 /       \
                v         v
             Relay B    Relay C
                |           |
                v           v
           Subscribers  Subscribers

## Connectivity

- [ ] Establish relay-to-relay QUIC connections
- [ ] Define relay identity
- [ ] Authenticate relays
- [ ] Handle relay disconnects
- [ ] Reconnect relays

## Remote Tracks

- [ ] Discover remote Tracks
- [ ] Subscribe to remote Tracks
- [ ] Forward Objects between relays
- [ ] Handle remote Track disappearance
- [ ] Avoid duplicate upstream subscriptions

---

# Phase 7: Distributed Routing

Subscribers should not need to know where a Track originated.

- [ ] Track discovery
- [ ] Track ownership
- [ ] Relay routing information
- [ ] Remote subscription propagation
- [ ] Subscription aggregation
- [ ] Avoid duplicate forwarding
- [ ] Select appropriate relay path
- [ ] Handle relay failures
- [ ] Recover subscriptions after reconnect

Example:

                    Austin
                       |
                  Track: X
                   /     \
                  v       v
                NYC     London
                 |         |
                 v         v
            Subscribers Subscribers

---

# Phase 8: Multi-Region Relay Network

Deploy real relays.

Initial topology:

    Austin <------> NYC
       \             /
        \           /
         \         /
          London

- [ ] Deploy Austin relay
- [ ] Deploy NYC relay
- [ ] Deploy London relay
- [ ] Connect relays
- [ ] Publisher connects to nearest relay
- [ ] Subscriber connects to nearest relay
- [ ] Discover remote Tracks
- [ ] Forward Tracks across regions
- [ ] Test cross-region latency
- [ ] Test relay failure
- [ ] Test relay reconnection
- [ ] Test different network topologies

---

# Phase 9: Video as an Application

Video should be implemented on top of the generic MoQ layer.

The core MoQ implementation should not need to understand video.

Target:

    Camera
       |
       v
    Encoder
       |
       v
    Encoded bytes
       |
       v
    MoQ Object
       |
       v
    Relay
       |
       v
    MoQ Object
       |
       v
    Decoder
       |
       v
    Video

- [ ] Create video Track
- [ ] Connect camera pipeline to Publisher
- [ ] Convert encoded video units to Objects
- [ ] Publish video Objects
- [ ] Forward video through relay
- [ ] Receive video Objects
- [ ] Decode video
- [ ] Display video
- [ ] Handle keyframes
- [ ] Handle late subscribers

---

# Phase 10: Audio

- [ ] Create audio Track
- [ ] Capture microphone
- [ ] Encode audio
- [ ] Publish audio Objects
- [ ] Receive audio
- [ ] Decode audio
- [ ] Synchronize audio/video
- [ ] Handle audio subscriptions

---

# Phase 11: Video Conferencing

Build the actual application.

- [ ] Rooms
- [ ] Participants
- [ ] Publish camera
- [ ] Publish microphone
- [ ] Subscribe to participants
- [ ] Join / leave
- [ ] Mute / unmute
- [ ] Camera on / off
- [ ] Screen sharing
- [ ] Participant presence
- [ ] Multiple video tracks

Target:

                     MoQ Relay
                   /     |     \
                  /      |      \
                 v       v       v
              User A  User B  User C

Each participant can publish and subscribe to multiple Tracks.

---

# Phase 12: Performance & Networking Experiments

Only benchmark once the system is functionally correct.

## Latency

- [ ] Publisher → Relay latency
- [ ] Relay → Subscriber latency
- [ ] End-to-end latency
- [ ] Cross-region latency
- [ ] Join latency

## Resource Usage

- [ ] CPU usage
- [ ] Memory usage
- [ ] Bandwidth
- [ ] Relay throughput
- [ ] Subscriber scaling
- [ ] Publisher scaling

## Network Conditions

- [ ] High RTT
- [ ] Packet loss
- [ ] Limited bandwidth
- [ ] Jitter
- [ ] Relay failure
- [ ] Reconnection

---

# Phase 13: WebRTC SFU Comparison

Compare the MoQ architecture against a WebRTC SFU.

## Architecture

- [ ] Compare protocol architecture
- [ ] Compare relay architecture
- [ ] Compare media pipeline
- [ ] Compare multi-region architecture

## Performance

- [ ] End-to-end latency
- [ ] Join latency
- [ ] CPU
- [ ] Memory
- [ ] Bandwidth
- [ ] Subscriber scaling
- [ ] Cross-region behavior
- [ ] Packet-loss behavior

## Architecture Questions

- [ ] How does each system handle fan-out?
- [ ] How does each system handle backpressure?
- [ ] How does each system handle late subscribers?
- [ ] How does each system handle relay failures?
- [ ] What changes when adding another relay?
- [ ] What work happens at each hop?

---

# Phase 14: Advanced MoQ / Networking

- [ ] Study current MoQ transport specification
- [ ] Study existing MoQ implementations
- [ ] Flow control
- [ ] Backpressure
- [ ] Object prioritization
- [ ] Object expiration
- [ ] Caching
- [ ] Late subscriber behavior
- [ ] Keyframe-aware subscriptions
- [ ] Relay load balancing
- [ ] Relay selection
- [ ] Failure recovery
- [ ] Distributed state
- [ ] Relay federation

---

# Phase 15: MoQ Ecosystem Contribution

The ultimate goal is to understand the protocol well enough to contribute upstream.

- [ ] Study existing Rust MoQ implementations
- [ ] Study `moq-lite`
- [ ] Study `moq-relay`
- [ ] Understand interoperability requirements
- [ ] Run interoperability tests
- [ ] Identify implementation gaps
- [ ] Identify protocol improvements
- [ ] Find an upstream issue to work on
- [ ] Submit a contribution
- [ ] Participate in MoQ ecosystem discussions

---

# Current Progress

## Completed

- QUIC connection setup
- TLS setup
- GStreamer setup
- Camera capture
- AV1 encoding
- H.264 encoding
- Codec-independent video encoder
- Basic bidirectional QUIC stream
- Client → Server data transfer
- Server → Client data transfer

## Current Milestone

**Generic QUIC Echo**

    Client
       |
       | message
       v
     Server
       |
       | same message
       v
     Client

- [ ] Implement single-message echo
- [ ] Verify echoed bytes match
- [ ] Support multiple messages on one stream
- [ ] Investigate message boundaries
- [ ] Implement application-level framing

## Next Milestone

**Generic Message Layer**

    QUIC
      |
      v
    Framing
      |
      v
    Messages
      |
      v
    MoQ

---

# Design Principles

1. **Generic first**

   MoQ should transport arbitrary data. Video is an application, not the core protocol.

2. **Understand before abstracting**

   Build the networking primitives first. Introduce abstractions only when their purpose becomes clear.

3. **Relays should be payload agnostic**

   The relay should not need to understand whether an Object contains video, audio, text, or arbitrary bytes.

4. **Learn the transport**

   Understand QUIC connections, streams, flow control, backpressure, and stream lifecycle rather than hiding everything behind a high-level API.

5. **Build incrementally**

   Every milestone should produce something that can be run, tested, and understood.

6. **Optimize after correctness**

   First make the protocol work. Then measure. Then optimize based on actual bottlenecks.

7. **Keep video separate from the core**

   The eventual goal is:

       Application
           |
       MoQ Pub/Sub
           |
       MoQ Transport
           |
          QUIC

   Video should plug into this stack rather than define it.
