pub trait MigrateTable {
    /// Generates a database schema in the form of a tuple. <br>
    /// Name is the name of the struct/table.<br>
    /// Called like: `**struct_name**::generate_schema()`
    fn generate_schema() -> (String, Vec<(String, String)>);
}
