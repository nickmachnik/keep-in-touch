//! The `table` mod contains structs that hold the actual
//! data written, stored and read by the application.

#[derive(Debug)]
struct Table {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Entry {
    field: Type,
}
