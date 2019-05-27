use std::collections::HashSet;
use std::collections::HashMap;

pub struct DB {
    dsn: &'static str,
    tables: HashMap<String, Vec<(String, String)>>,
}

impl DB {
    pub fn new(dsn: &'static str) -> DB {
        DB { dsn: dsn, tables: HashMap::new() }
    }

    pub fn connect(&self) {
        // TODO: use some other crate to connect to the db using the db's DSN
    }

    pub fn close(&self) {
        // TODO: use some other crate to sever connection to db
    }

    pub fn create_table(&mut self, schema: (String, Vec<(String, String)>)) -> Result<(), &'static str> {
        let name = schema.0; // named with underbar just to make compiler happy, eventually we'll be using it and that will change
        let fields = schema.1;
        let legal_types: HashSet<String> =
            vec!["String".to_string(), "u64".to_string(), "f64".to_string()]
                .into_iter()
                .collect();
        for f in fields.iter() {
            if !legal_types.contains(&f.1) {
                return Err(
                    "RSORM models can only contain the following types: u64, f64, and String",
                );
            }
        }

        // TODO: this function should now use the name and fields arguments to make a SQL call to create a table
        self.tables.insert(name, fields);
        Ok(())
    }
}
