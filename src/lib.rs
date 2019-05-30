extern crate proc_macro;
use serde::de::DeserializeOwned;
use std::collections::{HashMap, HashSet};
//use rusqlite::types::ToSql;
//use rusqlite::{params, Connection, NO_PARAMS};

pub struct DB {
    dsn: &'static str,
    tables: HashMap<String, Vec<(String, String)>>,
    conn: sqlite::Connection,
}

impl DB {
    pub fn new(dsn: &'static str) -> DB {
        DB {
            dsn: dsn,
            tables: HashMap::new(),
            conn: sqlite::open("testDB").unwrap(),
        }
    }

    pub fn close(self) {
        //self.conn.close().unwrap()
        // TODO: use some other crate to sever connection to db
    }

    pub fn create_table(
        &mut self,
        schema: (String, Vec<(String, String)>),
    ) -> Result<(), &'static str> {
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
        let ts = table_string(&name, &fields);
        if self.tables.contains_key(&name) {
            return Ok(());
        }
        self.conn.execute(&ts).unwrap();
        self.tables.insert(name, fields);
        Ok(())
    }

    pub fn insert<T>(&self, table: &str, object: &mut T) -> Result<(), String> {
        // called like this: db.insert("Model", modelinstance)
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        // we'll still need some kind of macro to generate the code to retrieve each field's values though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        Ok(())
    }

    pub fn select<T>(&self, table: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {

        // stub insert since insert isn't implemented on this branch
        self.conn.execute(format!("INSERT INTO {} VALUES ('Boris', 27, 'someday')", table)).unwrap();

        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }

        let q_string = format!("SELECT * FROM {}", table);
        let mut vals: Vec<String> = Vec::new();
        let _stmt = self.conn.iterate(&q_string, |pairs| {
            for &(_column, value) in pairs.iter() {
                vals.push(String::from(value.unwrap()));
            }
            true
        })
        .unwrap();

        let json: String = self.build_struct_json(table, vals);
        let object: T = serde_json::from_str(&json).unwrap();
        Ok(object)
    }

    fn build_struct_json(&self, table: &str, vals: Vec<String>) -> String {
        let mut json = String::from("{ ");
        let fields = self.tables.get(table).unwrap();
        let mut i = 0;
        for (ident, ty) in fields {
            json.push_str("\"");
            json.push_str(ident);
            json.push_str("\": ");
            if *ty == String::from("String") {
                json.push_str("\"");
            }
            json.push_str(&vals[i]);
            if *ty == String::from("String") {
                json.push_str("\"");
            }
            json.push_str(",");
            i += 1;
        }

        json.pop();
        json.push_str(" }");
        json 
    }
}

fn table_string(name: &String, fields: &Vec<(String, String)>) -> String {
        let mut values = String::from("");
        for f in fields {
            values.push_str(&format!(" {} {},", f.0, f.1));
        }

        values.pop();
        format!("create table if not exists {} ({} );", name, values)
    }
