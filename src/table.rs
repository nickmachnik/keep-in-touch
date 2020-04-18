//! The `table` mod contains structs that hold the actual
//! data written, stored and read by the application.

use chrono::{DateTime, Utc};
use colored::Colorize;
use hashbrown::HashMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    entries: HashMap<String, Entry>,
}

impl Table {
    fn new() -> Self {
        Table {
            entries: HashMap::new(),
        }
    }

    pub fn from_json(path: &Path) -> Self {
        let json_file_str = read_to_string(path).expect("Infile not found.");
        serde_json::from_str(&json_file_str).expect("Error while reading infile.")
    }

    pub fn to_json(&self, outpath: &Path) {
        let mut file =
            BufWriter::new(File::create(&outpath).expect("Error when creating outfile."));
        serde_json::to_writer(&mut file, self).expect("Error writing to outfile.");
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
            .for_each(|(_k, v)| v.update_remaining_time());
    }

    fn print_by_remaining_time(&self) {
        let mut res = self.entries.values().collect::<Vec<&Entry>>();
        res.sort_by(|a, b| a.remaining_time.cmp(&b.remaining_time));
        for e in res {
            e.print()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    pub name: String,
    // The chat interval in days
    pub interval: usize,
    last_contact: DateTime<Utc>,
    remaining_time: i64,
}

impl Entry {
    fn new(name: String, interval: usize, last_contact: DateTime<Utc>) -> Self {
        Entry {
            name,
            interval,
            last_contact,
            remaining_time: (interval as i64
                - Utc::now().signed_duration_since(last_contact).num_days()),
        }
    }

    fn update_remaining_time(&mut self) {
        self.remaining_time = self.interval as i64
            - Utc::now()
                .signed_duration_since(self.last_contact)
                .num_days();
    }

    fn print(&self) {
        let mut line = format!(
            "{:?}\t{:?}\t{:?}\t{:?}",
            self.name,
            self.interval,
            self.last_contact.date(),
            self.remaining_time
        );
        if self.remaining_time < 3 {
            line = line.red().on_black().to_string();
        } else if self.remaining_time > 10 {
            line = line.green().on_black().to_string();
        } else {
            line = line.magenta().on_black().to_string();
        }
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_remaining_time() {
        let mut e1 = Entry::new("Martin".to_string(), 30, Utc::now());
        e1.update_remaining_time();
        assert_eq!(30, e1.remaining_time);
    }

    #[test]
    fn test_update_through_table_par() {
        let remaining_daniel = 30 as i64
            - Utc::now()
                .signed_duration_since(Utc.ymd(2020, 3, 20).and_hms(12, 12, 12))
                .num_days();
        let e2 = Entry::new(
            "Daniel".to_string(),
            30,
            Utc.ymd(2020, 3, 20).and_hms(12, 12, 12),
        );
        let mut table = Table::new();
        table.add_entry(e2);
        table.update_entries_par();
        assert_eq!(
            remaining_daniel,
            table.entries.get("Daniel").unwrap().remaining_time
        );
    }

    #[test]
    fn test_print() {
        let e1 = Entry::new("Martin".to_string(), 30, Utc::now());
        let e2 = Entry::new(
            "Daniel".to_string(),
            30,
            Utc.ymd(2020, 3, 20).and_hms(12, 12, 12),
        );
        let e3 = Entry::new(
            "Thorben".to_string(),
            35,
            Utc.ymd(2020, 3, 20).and_hms(12, 12, 12),
        );
        let mut table = Table::new();
        table.add_entry(e2);
        table.add_entry(e1);
        table.add_entry(e3);
        table.print_by_remaining_time();
    }
}
