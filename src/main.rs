use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
mod lib;

#[allow(dead_code)]
#[derive(MigrateTable)]
struct Model {
    name: String,
    age: u64,
    birthday: u64,
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
    let db = lib::DB::new("some_dsn_here");
    db.create_table(Model::generate_schema()).unwrap();

    // Example of create_table returning an error when based a model struct containing an illegal type
    let result = db.create_table(BadModel::generate_schema());
    assert!(result.is_err());
}
