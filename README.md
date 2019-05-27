# rsorm
A SQL ORM written in Rust for Rust Programming 510 at Portland State University Spring 2019

This library seeks to provide a simple interface for interacting with SQLite databases.

## Usage
rsorm exposes a procedural macro that can be used to generate a table schema for your types that rsorm understands. To use the macro, list `migrate_table` and `migrate_table_derive` as dependencies in your `Cargo.toml`. Then, simply bring the two crates into scope and derive the `MigrateTable` trait for your model type:

```rust
use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;

#[derive(MigrateTable)]
struct Model {
    name: String,
    age: u64,
    birthday: String,
}
```

The `DB` type defined in `src/lib.rs` provides simple methods for connecting to a database, creating tables, inserting values, and retrieving values. The following illustrates a common use pattern: 

```rust
let mut db = lib::DB::new("sqlite:/opt/databases/mydb.sq3");
db.connect();
db.create_table(Model::generate_schema()).unwrap();

let mut obj = Model {
    name: String::from("boris"),
    age: 65,
    birthday: String::from("sometime"),
};

// inserts values defined in obj into the "Model" table
db.insert("Model", &mut obj).unwrap();

// populates obj_vec with the contents of the "Model" table
let mut obj_vec: Vec<Model> = Vec::new();
db.select("Model", &mut obj_vec).unwrap();

db.close();
```
