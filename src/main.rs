use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
mod lib;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(MigrateTable)]
struct Model{
    name: String,
    age: u64,
    birthday: String,
}

#[allow(dead_code)]
//#[derive(Debug)]
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

    let mut obj = Model {
        name: String::from("boris"),
        age: 65,
        birthday: String::from("sometime"),
    };

    // Example of create_table returning an error when passed a model struct containing an illegal type
    let result = db.create_table(BadModel::generate_schema());
    //assert!(result.is_err());
    
    
    
    
    //For Inserting items into the database, a user envokes the sql! macro
    //Since the macro reads in tokens directly, no rust object or refrences can be used
    //Any strings ot text data for the DB need to be wrapped in " "
    let result = db.insert("Model", sql!(name="Jordan", age=27, birthday="idk"));
    
    let mut obj_vec: Vec<Model> = Vec::new();

    //db.select("Model", &mut obj_vec).unwrap();

    db.close();
}
