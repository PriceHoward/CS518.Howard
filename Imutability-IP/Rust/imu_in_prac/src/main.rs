use std::thread;
use std::sync::{Arc, Mutex};
use imu_in_prac::{EventStore, Event, append_event, snapshot_at, history};

fn concurrency_demo() {
    let published: Arc<Mutex<Arc<EventStore>>> = Arc::new(Mutex::new(Arc::new(EventStore::new())));

    let mut handles = vec![];

    for version in 1..=3 {
        let published_clone = Arc::clone(&published);
        let handle = thread::spawn(move || {
            loop {
                let store = Arc::clone(&published_clone.lock().unwrap());
                if store.len() >= version {
                    let snapshot = snapshot_at(store.as_ref(), version);
                    println!("Reader at version {}: {:?}", version, snapshot);
                    break;
                }
            }
        });
        handles.push(handle);
    }

    let published_writer = Arc::clone(&published);
    let writer = thread::spawn(move || {
        let events = vec![
            Event::AddItem { id: 1, text: "Buy Book".to_string() },
            Event::AddItem { id: 2, text: "Read Book".to_string() },
            Event::CompleteItem { id: 1 },
            Event::RenameItem { id: 2, new_text: "Read Dune".to_string() },
            Event::DeleteItem { id: 1 },
            Event::CompleteItem{ id: 2 },
            Event::DeleteItem { id: 2 }
        ];

        for event in events {
            let current = Arc::clone(&published_writer.lock().unwrap());
            let next = Arc::new(append_event(current.as_ref(), event));
            let version = next.len();
            println!("Writer published version {}: {:?}", version, snapshot_at(&next, version));
            *published_writer.lock().unwrap() = next;
        }

        let final_store = Arc::clone(&published_writer.lock().unwrap());
        println!("\n---Full History---");
        let h = history(final_store.as_ref(), 1);
        for (version, state) in h {
            println!("version {}: {:?}", version, state);
        }
    });
    handles.push(writer);

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    concurrency_demo();
}