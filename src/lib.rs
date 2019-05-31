use std::collections::{HashMap, HashSet};
use sqlite;

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
            conn: sqlite::open(dsn).unwrap()
        }
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
    
    pub fn select<T>(&self, table: &str, object: &mut Vec<T>) -> Result<(), String> {
        // called like this: db.select("Model", modelinstance), where modelinstance is initilized to default values
        // we can use 'table' as a key to self.tables so that we know how to generate our query.
        // we'll need some kind of macro to generate the code to populate those fields with the values from the query result though...
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        Ok(())
    }
    pub fn delete(&self, table: &str, data:(Vec<String>, Vec<String>)) -> Result<(), String> {
        //called like this: db.delete("Model", sql!(field1=condition1, field2=condition2))
        // The 'table' argument will be used as a key to self.tables so that we know what fields object has
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }
        let ds = delete_string(table, data);
        self.conn.execute(&ds).unwrap();
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
    return format!("INSERT INTO {} {} VALUES {}", name, fields, values);
}

fn delete_string(name: &str, data: (Vec<String>, Vec<String>)) -> String {
    let mut conditions = String::from("(");
    for i in 0..(data.0).len() {
        conditions.push_str(&format!("{}={} and ", (data.0)[i], (data.1)[i]))
    }
    let trunc_val = conditions.len()-4;
    conditions.truncate(trunc_val);
    conditions.push(')');
    return format!("DELETE FROM {} WHERE {}", name, conditions);
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
 