//! Domain types for a single interactive cloud-gaming session.

/// Server-issued identifier for a game session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameSessionId(pub u128);

/// Lifecycle states shared by the session manager, worker, and client.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameSessionState {
    Requested,
    Assigned,
    Ready,
    Ended,
}

/// A game session has one player and one assigned game worker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameSession {
    pub id: GameSessionId,
    pub state: GameSessionState,
}

impl GameSession {
    pub fn requested(id: GameSessionId) -> Self {
        Self {
            id,
            state: GameSessionState::Requested,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameSession, GameSessionId, GameSessionState};

    #[test]
    fn new_session_starts_requested() {
        let session = GameSession::requested(GameSessionId(42));

        assert_eq!(session.id, GameSessionId(42));
        assert_eq!(session.state, GameSessionState::Requested);
    }
}
