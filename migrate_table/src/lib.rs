pub trait MigrateTable {
    fn generate_schema() -> (String, Vec<(String, String)>);
    fn insert_to_table<T>(object: T) -> String;
}
