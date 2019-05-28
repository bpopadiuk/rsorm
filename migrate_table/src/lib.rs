use proc_macro_hack::proc_macro_hack;

pub trait MigrateTable {
    fn generate_schema() -> (String, Vec<(String, String)>);
}

#[proc_macro_hack]
pub use migrate_table_derive::build_struct;
