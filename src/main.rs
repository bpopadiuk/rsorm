use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
use serde::Deserialize;
mod lib;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(MigrateTable, Deserialize, Debug)]
struct Model {
    name: String,
    age: u64,
    birthday: String,
}

#[allow(dead_code)]
#[derive(MigrateTable)]
struct BadModel {
    name: String,
    illegal: u8,
}

fn main() {
    // Here's a demonstration of what the MigrateTable macro is doing for us
    let (name, fields) = Model::generate_schema();
    println!("NAME: {:?}\nFIELDS: {:?}", name, fields);

    // Usually we'll just be calling it as an argument to the create_table() method though
    let mut db = lib::DB::new("some_dsn_here");

    db.create_table(Model::generate_schema()).unwrap();

    let inp = Model {
        name: "Boris".to_string(),
        age: 27,
        birthday: "someday".to_string(),
    };

    // Example of create_table returning an error when passed a model struct containing an illegal type
    let result = db.create_table(BadModel::generate_schema());
    assert!(result.is_err());

    //For Inserting items into the database, a user envokes the sql! macro
    //Since the macro reads in tokens directly, no rust object or refrences can be used
    //Any strings ot text data for the DB need to be wrapped in " "
    db.insert("Model", sql!(name = "Jordan", age = 8, birthday = "idk"))
        .unwrap();

    //Deleting items from the database looks just like inserting
    //All records that match the provided conditions will be deleted
    //SQLITE does not throw an error if no records match the conditions provided
    //Errors that arise will result from column names that are mispleed or don't exist
    //can allow specificy one condtion per table column
    db.delete("Model", sql!(name = "Jordan", age = 10)).unwrap();

    // This one should fail...
    let result2 = db.insert(
        "nonexistent",
        sql!(name = "Jordan", age = 8, birthday = "idk"),
    );
    assert!(result2.is_err());

    let mut out: Vec<Model> = Vec::new();
    db.select_all("Model", &mut out).unwrap();
    println!("IN:  {:?}\nOUT: {:?}", inp, out);
}
