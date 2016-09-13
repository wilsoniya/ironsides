#[deriving(Show)]
pub struct State {
    persistentState: PersistentState,
    volatileState: VolatileState,
    volatileLeaderState: Option<VolatileLeaderState>,
    role: MemberRole,
}

impl State {
    pub fn new() -> State {
        let persistent_state = PersistentState{
            currentTerm: 0,
            votedFor: None,
            log: Vec::new()
        };
        let volatile_state = VolatileState {
            commitIndex: 0,
            lastApplied: 0
        };
        State {
            persistentState: persistent_state,
            volatileState: volatile_state,
            volatileLeaderState: None,
            role: MemberRole::Follower
        }
    }
}

#[deriving(Show)]
struct PersistentState {
    currentTerm: i64,
    votedFor: Option<i32>,
    log: Vec<LogEntry>,
}

#[deriving(Show)]
struct VolatileState { 
    commitIndex: i64,
    lastApplied: i64,
}

#[deriving(Show)]
struct VolatileLeaderState { 
    nextIndex: Option<Vec<i64>>,
    lastApplied: Option<Vec<i64>>,
}

#[deriving(Show)]
struct LogEntry {
    name: String,
    value: String, 
}

enum MemberRole {
    Follower,
    Candidate,
    Leader
}
