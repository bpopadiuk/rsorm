# rsorm
A SQL ORM written in Rust for Rust Programming 510 at Portland State University Spring 2019

This library seeks to provide a very simple interface for interacting with SQLite databases. As of this writing most available SQLite crates require some amount of raw sql query strings and their database operation symantics can be complicated. rsorm seeks to distill db interactions to the bare minimum of complexity. While this can limit flexibility to some degree, it makes getting started working with SQLite very easy for many of the most common use cases. 

rsorm provides a small collection of macros and a single `DB` type that exposes the primary functionality of the crate. `DB` wraps the [sqlite](https://crates.io/crates/sqlite) crate and abstracts out much of the complexity required to interact with a database.

## Dependencies
Due to current Rust restrictions on custom procedural macros, `rsorm` contains two sub-crates that your project will need to depend on as well. Your project will also need `serde`, which `rsorm` uses for deserialization. Place the following in your `Cargo.toml`:

```
[dependencies]
rsorm = { git = "https://github.com/bpopadiuk/rsorm" }
migrate_table = { git = "https://github.com/bpopadiuk/rsorm" }
migrate_table_derive = { git = "https://github.com/bpopadiuk/rsorm" }
serde = { version = "1.0", features = ["derive"] }
```

## Usage
rsorm exposes a procedural macro that can be used to generate a table schema for your types that rsorm understands. To use the macro, simply bring the two crates into scope and derive the `MigrateTable` trait for your model type. Your model type will need to derive `serde`'s `Deserialize` trait as well:

```rust
use rsorm::{sql, DB};
use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
use serde::Deserialize;

#[derive(MigrateTable, Deserialize)]
struct Person {
    name: String,
    age: u64,
    birthday: String,
}
```

The `DB` type defined in `src/lib.rs` provides simple methods for connecting to a database, creating tables, inserting values, and retrieving values. The following illustrates common use patterns: 

```rust
let mut db = rsorm::DB::new("sqlite:/opt/databases/mydb.sq3");
db.create_table(Person::generate_schema()).unwrap();

// insert an instance of the Person type
db.insert("Person", sql!(name = "Jordan", age = 8, birthday = "01/01/1992"))
    .unwrap();

// delete an instance of the Person type. Bad fields trigger error, non-existent instances are silently ignored
db.delete("Person", sql!(name = "Jordan", age = 8))
    .unwrap();

// collect all records into a vector of Person
let mut records: Vec<Person> = Vec::new();
db.select_all("Person", &mut records).unwrap();

// collect all records matching a given condition
let mut filtered: Vec<Person> = Vec::new();
db.select_where("Person", &mut filtered, sql!(name = "Boris"))
    .unwrap();
```
