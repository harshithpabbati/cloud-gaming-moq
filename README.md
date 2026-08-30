# cloud-gaming-moq

An experimental Rust cloud-gaming system that evaluates QUIC and Media over
QUIC (MoQ) for low-latency game streaming.

The product goal is a player controlling a remotely running game with
predictable input-to-photon latency.

## Direction

The core unit is a single `GameSession`: one authorized player, one assigned
game worker, bidirectional input/media transport, and explicit lifecycle
management.

```text
                       control, input, feedback
  Player client  <-------------------------------->  Session gateway
       ^                                                    |
       | video and audio                                    | assignment
       +----------------------------------------------  Game worker
                                                        game + GPU encoder
```

MoQ is a deliberate part of the project name and a technology to evaluate for
media distribution. It is not assumed to be the correct transport for the
initial interactive player-to-worker path. That decision will be made using
latency, loss-recovery, and implementation-complexity measurements.

## Principles

- A `GameSession`, not a topic or channel, is the primary domain object.
- Control, input, video, audio, and feedback have distinct delivery rules.
- Stale input and video must be dropped rather than queued.
- Game workers should be placed near players; media relays are not on the
  initial critical path.
- Every latency-sensitive stage uses a monotonic timestamp.
- The first success criterion is one playable local session, then LAN, then
  regional deployment.

## Transport Model

| Traffic         | Expected delivery                       | Candidate transport              |
| --------------- | --------------------------------------- | -------------------------------- |
| Session control | Reliable and ordered                    | QUIC bidirectional stream        |
| Player input    | Sequenced; stale packets dropped        | QUIC datagrams                   |
| Video           | Frame-aware; stale delta frames dropped | Direct QUIC or MoQ experiment    |
| Audio           | Low jitter; late audio dropped          | Dedicated QUIC media path        |
| Client feedback | Best effort                             | QUIC datagrams or control stream |

Encoded media will not use a generic reliable application message queue.

## Current State

The former generic pub/sub relay, camera experiments, development
certificates, and associated tests have been removed. The repository is now a
clean cloud-gaming foundation containing only session domain primitives.

There is no runnable streaming demo yet.

## Development

Build and test the foundation:

```sh
cargo test
```

## Roadmap

### Phase 1: Session and Control Protocol

- [x] Rename the project to `cloud-gaming-moq`
- [x] Remove the generic pub/sub implementation
- [x] Introduce `GameSessionId` and session lifecycle states
- [ ] Define session creation, join, ready, end, and error control messages
- [ ] Define player identity, session authorization, and worker assignment
- [ ] Add protocol versioning and invalid-message tests

### Phase 2: Local Interactive Slice

- [ ] Build a deterministic game-worker scene with keyboard/controller input
- [ ] Create a native client that captures input and renders decoded frames
- [ ] Define sequenced, timestamped input packets
- [ ] Deliver input over QUIC datagrams and discard stale input
- [ ] Capture and hardware-encode H.264 video where available
- [ ] Implement frame metadata, bounded queues, frame dropping, and keyframe
      requests
- [ ] Demonstrate a local player controlling the worker at 60 fps

### Phase 3: Measurement and Recovery

- [ ] Timestamp input capture, worker receive, simulation, capture, encode,
      send, receive, decode, and render
- [ ] Report p50, p95, and p99 end-to-end and per-stage latency
- [ ] Report frame rate, bitrate, dropped input, dropped frames, and queue
      depth
- [ ] Test latency, loss, jitter, decoder recovery, and worker/client failure
- [ ] Set hard limits for packet sizes, queues, sessions, and memory

### Phase 4: Audio and Adaptation

- [ ] Add low-latency Opus game audio
- [ ] Add client audio playback and bounded jitter buffering
- [ ] Adapt bitrate, resolution, and frame rate from congestion and feedback
- [ ] Add on-demand keyframe recovery

### Phase 5: Worker and Regional Deployment

- [ ] Add a worker registry, health checks, capacity, and session assignment
- [ ] Separate gateway and worker processes
- [ ] Validate LAN deployment before remote deployment
- [ ] Add region-aware placement and admission control

### Phase 6: MoQ Evaluation and Distribution

- [ ] Compare direct QUIC and MoQ for the interactive media path
- [ ] Add spectator, recording, or broadcast pipelines where fanout is needed
- [ ] Use MoQ relay distribution only where measurements justify it
