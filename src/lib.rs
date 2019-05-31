use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};

pub struct DB {
    dsn: &'static str,
    tables: HashMap<String, Vec<(String, String)>>,
    conn: Connection,
}

impl DB {
    pub fn new(dsn: &'static str) -> DB {
        DB {
            dsn: dsn,
            tables: HashMap::new(),
            conn: Connection::open("testDB.db").unwrap(),
        }
    }

    pub fn close(self) {
        self.conn.close().unwrap()
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
        self.conn.execute(&ts, NO_PARAMS).unwrap();
        self.tables.insert(name, fields);
        Ok(())
    }

    pub fn insert(&self, table: &str, data: (String, String)) -> Result<(), String> {
        // called like this: db.insert("Model", sql!(field1= data1, field2= data2, field3=data3))
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        // we'll still need some kind of macro to generate the code to retrieve each field's values though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        let is = insert_string(table, data);
        self.conn.execute(&is, NO_PARAMS).unwrap();
        Ok(())
    }
  
    pub fn select<T>(&self, table: &str, object: &mut Vec<T>) -> Result<(), String> {
        // called like this: db.select("Model", modelinstance), where modelinstance is initilized to default values
        // we can use 'table' as a key to self.tables so that we know how to generate our query.
        // we'll need some kind of macro to generate the code to populate those fields with the values from the query result though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        Ok(())
    }
}

fn table_string(name: &String, fields: &Vec<(String, String)>) -> String {
        let mut values = String::from("");
        for f in fields {
            values.push_str(&format!(" {} {},", f.0, f.1));
        }

        values.pop();
        format!("CREATE TABLE IF NOT EXISTS {} ({} );", name, values)
    }

fn insert_string(name: &str, data: (String, String)) -> String {
    return format!("INSERT INTO {} {} VALUES {}", name, data.0, data.1)
}


//macro for parsing the options for the insert comand
//reads in tokes in the tokens directly, this can not handle any objects or refrenceces
//tthe data will be entered directly as is, additonally the l
#[macro_export]
macro_rules! sql {
    ($($x:tt = $y:tt), *) => {
        {
        
            let mut s1: String = "(".to_string();
            let mut s2: String = "(".to_string();
            $(
                s1.push_str(stringify!($x,));
                let z = stringify!( $y,).replace("\"","'");
                s2.push_str(&z);
            )*
            s1.pop();
            s2.pop();
            s1.push_str(")");
            s2.push_str(")");
            (s1, s2)
        }
    };
} 
 