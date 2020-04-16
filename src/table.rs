//! The `table` mod contains structs that hold the actual
//! data written, stored and read by the application.

use chrono::{Date, Duration, Utc};
use rayon::prelude::*;

#[derive(Debug)]
struct Table {
    entries: Vec<Entry>,
}

impl Table {
    fn new() -> Self {
        Table {
            entries: Vec::new(),
        }
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn update_entries_par(&mut self) {
        self.entries
            .par_iter_mut()
            .for_each(|e| e.update_remaining_time());
        self.entries
            .sort_by(|a, b| b.remaining_time.cmp(&a.remaining_time));
    }
}

#[derive(Debug)]
struct Entry {
    pub name: String,
    // The chat interval in days
    pub interval: Duration,
    last_contact: Date<Utc>,
    remaining_time: i64,
}

impl Entry {
    fn new(name: String, interval: Duration, last_contact: Date<Utc>) -> Self {
        Entry {
            name,
            interval,
            last_contact,
            remaining_time: (interval.num_days()
                - Utc::now()
                    .date()
                    .signed_duration_since(last_contact)
                    .num_days()),
        }
    }

    fn update_remaining_time(&mut self) {
        self.remaining_time = self.interval.num_days()
            - Utc::now()
                .date()
                .signed_duration_since(self.last_contact)
                .num_days();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaining_time() {
        let mut e1 = Entry::new("Martin".to_string(), Duration::days(30), Utc::now().date());
        assert_eq!(30, e1.remaining_time);
        e1.update_remaining_time();
        assert_eq!(30, e1.remaining_time);
    }
}
