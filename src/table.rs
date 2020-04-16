//! The `table` mod contains structs that hold the actual
//! data written, stored and read by the application.

use chrono::{Date, Duration, Utc};
use hashbrown::HashMap;
use rayon::prelude::*;

#[derive(Debug)]
struct Table {
    entries: HashMap<String, Entry>,
}

impl Table {
    fn new() -> Self {
        Table {
            entries: HashMap::new(),
        }
    }

    fn add_entry(&mut self, entry: Entry) {
        if self.entries.contains_key(&entry.name) {
            error!("A friend with name {:?} already exists. Please choose a different name or modify the existing entry", entry.name);
        } else {
            self.entries.insert(entry.name.clone(), entry);
        }
    }

    fn update_entries_par(&mut self) {
        self.entries
            .par_iter_mut()
            .for_each(|(k, v)| v.update_remaining_time());
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
