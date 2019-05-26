use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;
mod lib;

#[allow(dead_code)]
#[derive(MigrateTable)]
struct Model{
    name: String,
    age: u64,
    birthday: u64,
}

fn main() {
    let (name, fields) = Model::generate_schema();
    let db = lib::DB::new("some_dsn_here");
    db.create_table(&fields).unwrap();
    println!("NAME: {:?}\nFIELDS: {:?}", name, fields)
}
