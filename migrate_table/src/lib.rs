pub trait MigrateTable {
    fn generate_schema() -> (String, Vec<(String, String)>);
}
