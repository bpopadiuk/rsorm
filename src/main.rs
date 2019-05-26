use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;

#[derive(MigrateTable)]
struct Model{
    name: String,
    age: u64,
    birthday: u64,
}

fn main() {
    let (name, fields) = Model::generate_schema();
    println!("NAME: {:?}\nFIELDS: {:?}", name, fields)
}
