#[cfg(test)]
mod tests {
    use imu_in_prac::{EventStore, Event, append_event, snapshot_at};

    #[test]
    fn old_version_unchanged() {
        let s0 = EventStore::new();
        let s1 = append_event(&s0, Event::AddItem { id: 1, text: "Buy milk".to_string() });
        assert_eq!(snapshot_at(&s0, 0).len(), 0);
        assert_eq!(snapshot_at(&s1, 1).len(), 1);
    }

    #[test]
    fn non_interference() {
        let s0 = EventStore::new();
        let s1 = append_event(&s0, Event::AddItem { id: 1, text: "Buy milk".to_string() });
        let before = snapshot_at(&s1, 1).len();
        let _s2 = append_event(&s1, Event::AddItem { id: 2, text: "Read book".to_string() });
        let after = snapshot_at(&s1, 1).len();
        assert_eq!(before, after);
    }
}