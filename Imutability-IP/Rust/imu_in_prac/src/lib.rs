use std::collections::HashMap;
use std::sync::Arc;

const CHECKPOINT_LENGTH: usize = 3; // Global const for the length of versions until snapshot checkpoint is made.

pub type State = HashMap<u32, TodoItem>;

#[derive(Debug, Clone)]
pub enum Event {
    AddItem { id: u32, text: String },
    CompleteItem { id: u32 },
    RenameItem { id: u32, new_text: String },
    DeleteItem { id: u32 },
}

// The object that maintaisns the ID and the text of the tasks.
#[derive(Debug, Clone)]
pub struct TodoItem {
    pub text: String,
    pub completed: bool,
}

// The Object that stores all of the tasks.
#[derive(Debug, Clone)]
pub struct EventStore {
    events: Arc<Vec<Event>>,
    checkpoints: Arc<Vec<(usize, State)>>,
}

// Object Function Definitions
impl EventStore {
    pub fn new() -> EventStore {
        EventStore { 
            events: Arc::new(Vec::new()),
            checkpoints: Arc::new(Vec::new()),
        }
        
    }
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

// Function used to add new events into the store. Using a match, it checks for which event was passed in.
fn apply_event(mut state: State, event: &Event) -> State {
    match event {
        Event::AddItem {id, text} => {
            state.insert(*id, TodoItem{text: text.clone(), completed: false});
        }
        Event::CompleteItem{id}=>{
            if let Some(item) = state.get_mut(id) {
                item.completed = true;
            }
        }
        Event::RenameItem{id, new_text}=>{
            if let Some(item) = state.get_mut(id) {
                item.text = new_text.clone();
            }
        }
        Event::DeleteItem{id}=>{
            state.remove(id);
        }
    }
    state
}

// Computes the newest snapshot.
fn compute_snapshot(events: &Vec<Event>, version: usize) -> State {
    let mut state = State::new();
    for event in events.iter().take(version) {
        state = apply_event(state, event);
    }
    state
}

// Adds event into the Event Store object. Also creates snapshot if event is a multiple of 3.
pub fn append_event(store: &EventStore, event: Event) -> EventStore {
    let mut events = (*store.events).clone();
    events.push(event);
    let mut checkpoints = (*store.checkpoints).clone();
    if events.len() % CHECKPOINT_LENGTH == 0 {
        let version = events.len();
        let state = compute_snapshot(&events, version);
        checkpoints.push((version, state));
    }
    EventStore { 
        events: Arc::new(events),
        checkpoints: Arc::new(checkpoints)
    }
}

// Returns the state of the Version passed into the function.
pub fn snapshot_at(store: &EventStore, version: usize) -> State {
    let mut correct_checkpoint: Option<&(usize, State)> = None;
    for checkpoint in store.checkpoints.iter() {
        let checkpoint_version = checkpoint.0;
        let checkpoint_is_before_v = checkpoint_version <= version;

        if checkpoint_is_before_v {
            correct_checkpoint = Some(checkpoint);
        }
    }
    match correct_checkpoint {
        Some(checkpoint) => {
            let checkpoint_version = checkpoint.0;
            let checkpoint_state = &checkpoint.1;

            let mut state = checkpoint_state.clone();
            let events_to_skip = checkpoint_version;
            let events_to_replay = version - checkpoint_version;

            for event in store.events.iter().skip(events_to_skip).take(events_to_replay) {
                state = apply_event(state, event);
            }
            state
        }
        None => {
            compute_snapshot(&store.events, version)
        }
    }
}

// Returns a Vector of version_vec(s) that the list has gone through.
pub fn history(store: &EventStore, step_through: usize) -> Vec<(usize, State)> {
    let mut version_vec = Vec::new();
    let total_versions = store.events.len();
    let safe_step = step_through.max(1);

    let mut version = 0;
    while version <= total_versions{
        let state_v = snapshot_at(store, version);
        version_vec.push((version, state_v));
        version += safe_step;
    }
    version_vec
}