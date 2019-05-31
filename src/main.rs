use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
use serde::Deserialize;
mod lib;

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

    let mut inp = Model {
        name: "Boris".to_string(),
        age: 27,
        birthday: "someday".to_string(),
    };

    // Example of create_table returning an error when passed a model struct containing an illegal type
    let result = db.create_table(BadModel::generate_schema());
    assert!(result.is_err());

    // This one should fail...
    let result2 = db.insert("nonexistent", &mut inp);
    assert!(result2.is_err());

    let mut out: Vec<Model> = Vec::new();
    db.select_all("Model", &mut out).unwrap();
    println!("IN:  {:?}\nOUT: {:?}", inp, out);
}
