//! The `table` mod contains structs that hold the actual
//! data written, stored and read by the application.

use chrono::{DateTime, Utc};
use colored::Colorize;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::BufWriter;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsuspendedEntry {
    name: String,
}

impl fmt::Display for UnsuspendedEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name {} it not currently suspended.", self.name)
    }
}

impl error::Error for UnsuspendedEntry {
    fn description(&self) -> &str {
        "Entry is not suspended."
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingEntry {
    name: String,
}

impl fmt::Display for ExistingEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name {} already used", self.name)
    }
}

impl error::Error for ExistingEntry {
    fn description(&self) -> &str {
        "Name already used"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingEntry {
    name: String,
}

impl fmt::Display for MissingEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name {} not in list", self.name)
    }
}

impl error::Error for MissingEntry {
    fn description(&self) -> &str {
        "Name not in list"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub entries: HashMap<String, Entry>,
    pub suspended_entries: HashSet<String>,
    // thresholds for highlighting
    t1: i64,
    t2: i64,
    t3: i64,
}

impl Table {
    pub fn new() -> Self {
        Table {
            entries: HashMap::new(),
            suspended_entries: HashSet::new(),
            t1: 0,
            t2: 3,
            t3: 10,
        }
    }

    pub fn from_json(path: &Path) -> Result<Self, Box<dyn error::Error>> {
        let json_file_str = read_to_string(path)?;
        let data = serde_json::from_str(&json_file_str)?;
        Ok(data)
    }

    pub fn to_json(&self, outpath: &Path) {
        let mut file =
            BufWriter::new(File::create(&outpath).expect("Error when creating outfile."));
        serde_json::to_writer(&mut file, self).expect("Error writing to outfile.");
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<(), ExistingEntry> {
        if self.entries.contains_key(&entry.name) {
            Err(ExistingEntry { name: entry.name })
        } else {
            self.entries.insert(entry.name.clone(), entry);
            Ok(())
        }
    }

    pub fn remove_entry(&mut self, name: String) -> Result<(), MissingEntry> {
        if self.entries.remove(&name).is_none() {
            Err(MissingEntry { name })
        } else {
            Ok(())
        }
    }

    pub fn suspend_entry(&mut self, name: String) -> Result<(), MissingEntry> {
        if !self.entries.contains_key(&name) {
            Err(MissingEntry { name })
        } else if !self.suspended_entries.contains(&name) {
            Err()
        } else {
            Ok(())
        }
    }

    pub fn update_entries_par(&mut self) {
        self.entries
            .par_iter_mut()
            .for_each(|(_k, v)| v.update_remaining_time());
    }

    pub fn print_by_remaining_time(&self) {
        let mut res = self.entries.values().collect::<Vec<&Entry>>();
        res.sort_by(|a, b| a.remaining_time.cmp(&b.remaining_time));
        println!(
            "{0: <15}  {1: <15}  {2: <15}  {3: <15}",
            "Name".white().on_black().bold(),
            "Remaining".white().on_black().bold(),
            "Last".white().on_black().bold(),
            "Interval".white().on_black().bold()
        );
        for e in res {
            e.print(self.t1, self.t2, self.t3)
        }
    }
}

#[derive(Debug)]
pub struct EntryVec(Vec<Entry>);

impl core::ops::Deref for EntryVec {
    type Target = Vec<Entry>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for EntryVec {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl EntryVec {
    pub fn sort_by_time_ascending(&mut self) {
        self.sort_by(|a, b| a.remaining_time.cmp(&b.remaining_time));
    }

    pub fn sort_by_time_descending(&mut self) {
        self.sort_by(|a, b| b.remaining_time.cmp(&a.remaining_time));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    // The chat interval in days
    pub interval: usize,
    pub last_contact: DateTime<Utc>,
    remaining_time: i64,
}

impl Entry {
    pub fn new(name: String, interval: usize, last_contact: DateTime<Utc>) -> Self {
        Entry {
            name,
            interval,
            last_contact,
            remaining_time: (interval as i64
                - Utc::now().signed_duration_since(last_contact).num_days()),
        }
    }

    pub fn update_remaining_time(&mut self) {
        self.remaining_time = self.interval as i64
            - Utc::now()
                .signed_duration_since(self.last_contact)
                .num_days();
    }

    fn print(&self, t1: i64, t2: i64, t3: i64) {
        let mut line = format!(
            "{0: <15}  {1: <15}  {2: <15}  {3: <15}",
            self.name,
            self.remaining_time,
            self.last_contact.date(),
            self.interval,
        );
        if self.remaining_time < t1 {
            line = line.red().on_black().to_string();
        } else if self.remaining_time < t2 {
            line = line.yellow().on_black().to_string();
        } else if self.remaining_time > t3 {
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
    fn test_entry_vec_sort_time_descending() {
        let e1 = Entry::new("Martin".to_string(), 30, Utc::now());
        let e2 = Entry::new(
            "Daniel".to_string(),
            30,
            Utc.ymd(2020, 3, 20).and_hms(12, 12, 12),
        );
        let e3 = Entry::new(
            "Baniel".to_string(),
            30,
            Utc.ymd(2020, 5, 20).and_hms(12, 12, 12),
        );
        let mut entries = EntryVec(vec![e2, e1, e3]);
        entries.sort_by_time_descending();
        assert_eq!(
            entries.iter().map(|e| e.remaining_time).collect::<Vec<_>>()[0],
            30
        );
    }

    #[test]
    fn test_entry_vec_sort_time_ascending() {
        let e1 = Entry::new("Martin".to_string(), 30, Utc::now());
        let e2 = Entry::new(
            "Daniel".to_string(),
            30,
            Utc.ymd(2020, 3, 20).and_hms(12, 12, 12),
        );
        let e3 = Entry::new(
            "Baniel".to_string(),
            30,
            Utc.ymd(2020, 5, 20).and_hms(12, 12, 12),
        );
        let mut entries = EntryVec(vec![e2, e1, e3]);
        entries.sort_by_time_ascending();
        assert_eq!(
            entries.iter().map(|e| e.remaining_time).collect::<Vec<_>>()[2],
            30
        );
    }

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
        table.add_entry(e2).unwrap();
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
        table.add_entry(e2).unwrap();
        table.add_entry(e1).unwrap();
        table.add_entry(e3).unwrap();
        table.print_by_remaining_time();
    }
}
