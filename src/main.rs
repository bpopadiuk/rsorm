use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;

#[derive(MigrateTable)]
struct Model{
    name: String,
    age: u64,
    birthday: u64,
}

fn main() {
    Model::migrate_table();
}
