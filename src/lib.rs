extern crate proc_macro;
use serde::de::DeserializeOwned;
use sqlite;
use std::collections::{HashMap, HashSet};

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
            conn: sqlite::open(dsn).unwrap(),
        }
    }

    pub fn create_table(
        &mut self,
        schema: (String, Vec<(String, String)>),
    ) -> Result<(), &'static str> {
        let name = schema.0;
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

        let ts = self.table_string(&name, &fields);
        if self.tables.contains_key(&name) {
            return Ok(());
        }
        self.conn.execute(&ts).unwrap();
        self.tables.insert(name, fields);
        Ok(())
    }

    pub fn insert(&self, table: &str, data: (Vec<String>, Vec<String>)) -> Result<(), String> {
        // called like this: db.insert("Model", sql!(field1= data1, field2= data2, field3=data3))
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        // we'll still need some kind of macro to generate the code to retrieve each field's values though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        let is = insert_string(table, data);
        self.conn.execute(&is).unwrap();
        Ok(())
    }

    pub fn select_where<T>(
        &self,
        table: &str,
        objects: &mut Vec<T>,
        data: (Vec<String>, Vec<String>),
    ) -> Result<(), String>
    where
        T: DeserializeOwned,
    {
        // called like this: db.insert("Model", sql!(field1= data1, field2= data2, field3=data3))
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        // we'll still need some kind of macro to generate the code to retrieve each field's values though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        let q_string = where_string(table, data);
        self.select_query(table, q_string, objects)
    }

    pub fn select_all<T>(&self, table: &str, objects: &mut Vec<T>) -> Result<(), String>
    where
        T: DeserializeOwned,
    {
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }

        let q_string = format!("SELECT * FROM {}", table);
        self.select_query(table, q_string, objects)
    }

    pub fn delete(&self, table: &str, data: (Vec<String>, Vec<String>)) -> Result<(), String> {
        //called like this: db.delete("Model", sql!(field1=condition1, field2=condition2))
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        let ds = delete_string(table, data);
        self.conn.execute(&ds).unwrap();
        Ok(())
    }

    fn build_struct_json(&self, table: &str, vals: &[String]) -> String {
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

    fn table_string(&self, name: &String, fields: &Vec<(String, String)>) -> String {
        let mut values = String::from("");
        for f in fields {
            values.push_str(&format!(" {} {},", f.0, f.1));
        }

        values.pop();
        format!("CREATE TABLE IF NOT EXISTS {} ({} );", name, values)
    }

    fn select_query<T>(
        &self,
        table: &str,
        q_string: String,
        objects: &mut Vec<T>,
    ) -> Result<(), String>
    where
        T: DeserializeOwned,
    {
        let mut vals: Vec<String> = Vec::new();
        // this query pattern was taken from the sqlite crate docs: https://docs.rs/sqlite/0.24.1/sqlite/
        let _stmt = self
            .conn
            .iterate(&q_string, |pairs| {
                for &(_column, value) in pairs.iter() {
                    vals.push(String::from(value.unwrap()));
                }
                true
            })
            .unwrap();

        let chunks = vals.chunks(self.tables.get(table).unwrap().len());
        for c in chunks {
            let json: String = self.build_struct_json(table, c);
            let object: T = serde_json::from_str(&json).unwrap();
            objects.push(object);
        }

        Ok(())
    }
}

fn insert_string(name: &str, data: (Vec<String>, Vec<String>)) -> String {
    let mut fields = String::from("(");
    let mut values = String::from("(");
    for i in 0..(data.0).len() {
        fields.push_str(&format!("{},", (data.0)[i]));
        values.push_str(&format!("{},", (data.1)[i]));
    }
    fields.pop();
    values.pop();
    fields.push(')');
    values.push(')');
    format!("INSERT INTO {} {} VALUES {}", name, fields, values)
}

fn delete_string(name: &str, data: (Vec<String>, Vec<String>)) -> String {
    let conditions = build_conditions(data);
    format!("DELETE FROM {} WHERE {}", name, conditions)
}

fn where_string(name: &str, data: (Vec<String>, Vec<String>)) -> String {
    let conditions = build_conditions(data);
    format!("SELECT * FROM {} WHERE {}", name, conditions)
}

fn build_conditions(data: (Vec<String>, Vec<String>)) -> String {
    let mut conditions = String::from("(");
    for i in 0..(data.0).len() {
        conditions.push_str(&format!("{}={} and ", (data.0)[i], (data.1)[i]))
    }
    let trunc_val = conditions.len() - 4;
    conditions.truncate(trunc_val);
    conditions.push(')');
    conditions
}

//macro that parses user options for a sql! command
//will parse tokens in the form of "field1 = value1, field2=value2, field3=value3"
//returns a tuple of string vectors, one for fields, one for values
#[macro_export]
macro_rules! sql {
    ($($x:tt = $y:tt), *) => {
        {
            let mut fields:Vec<String> = Vec::new();
            let mut data: Vec<String> = Vec::new();
            $(
                fields.push(stringify!($x).to_string());
                let z = stringify!($y).replace("\"","'");
                data.push(z);
            )*
            (fields, data)
        }
    };
}
