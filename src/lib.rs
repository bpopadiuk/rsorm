pub struct DB {
    dsn: &'static str
}

impl DB {
    pub fn new(dsn: &'static str) -> DB {
        DB{dsn: dsn}
    }

    pub fn connect(&self) {
        // TODO: use some other crate to connect to the db using the db's DSN
    }

    pub fn close(&self) {
        // TODO: use some other crate to sever connection to db
    }

    pub fn create_table(&self, fields: &Vec<(String, String)>) -> Result<(), &'static str> {
        let legal_types: std::collections::HashSet<String> = vec!("String".to_string(), "u64".to_string(), "f64".to_string()).into_iter().collect();
        for f in fields {
            if !legal_types.contains(&f.1) {
                return Err("RSORM models can only contain the following types: u64, f64, and String")
            } 
        }

        // TODO: this function should now use the name and fields arguments to make a SQL call to create a table
        Ok(())
    }
}
