struct State {
    persistentState: PersistentState,
    volatileState: VolatileState,
    volatileLeaderState: Option<VolatileLeaderState>,
    role: MemberRole,
}


struct PersistentState {
    currentTerm: i64,
    votedFor: Option<i32>,
    log: Vec<LogEntry>,
}

struct VolatileState { 
    commitIndex: i64,
    lastApplied: i64,
}

struct VolatileLeaderState { 
    nextIndex: Option<Vec<i64>>,
    lastApplied: Option<Vec<i64>>,
}

struct LogEntry {
    name: String,
    value: String, 
}

enum MemberRole {
    Follower,
    Candidate,
    Leader
}
