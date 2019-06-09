extern crate proc_macro;
extern crate serde;
use serde::de::DeserializeOwned;
use sqlite;
use std::collections::{HashMap, HashSet};

///
/// A simple **sqlite** ORM.<br>
/// A SQL Table is defined with a struct derived with the `MigrateTable` trait.<br>
/// Insertions, Deletions, and Where clauses need to be wrapped with the `sql!` macro.<br>
///
///
///

pub struct DB {
    dsn: &'static str,
    tables: HashMap<String, Vec<(String, String)>>,
    conn: sqlite::Connection,
}
 
impl DB {
    ///
    /// Instantiate a DB isntance
    ///
    /// # Arguments
    ///
    /// * `dsn` - a filepath locating the database file, will create if it doesn't exist.
    ///
    pub fn new(dsn: &'static str) -> DB {
        DB {
            dsn: dsn,
            tables: HashMap::new(),
            conn: sqlite::open(dsn).unwrap(),
        }
    }
    ///
    /// Create a table to insert into the database.<br>
    /// Requires a struct with the `MigrateTable` trait derived.<br>
    ///
    /// # Arguments
    ///
    /// * `schema` - the result of `generate_schema()` called on a struct derived with `MigrateTable` 
    ///
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
    
    ///
    ///Inserts into the specifed table, the data provided
    ///
    /// # Arguments
    /// * `table` - The name of a previously created table, as a string
    /// * `data` - The data that is to be entered into the database.
    ///    called with the `sql` macro
    ///
    pub fn insert(&self, table: &str, data: (Vec<String>, Vec<String>)) -> Result<(), String> {
    
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }

        if self.invalid_fields(table, &data) {
            return Err(format!("Invalid column/s for db table: {}", table));
        }

        let is = insert_string(table, data);
        self.conn.execute(&is).unwrap();
        Ok(())
    }

    ///
    /// Select all records from table that match specified conditions 
    ///
    /// # Arguments
    /// * `table` - The name of a previously created table, as a string
    /// * `object` - a refrence to a generic emoty vector that will be populated with the records returned from the database.
    /// * `data` - the conditions that will be matched against for selection, called with the `sql` macro.
    /// 
    pub fn select_where<T>(
        &self,
        table: &str,
        objects: &mut Vec<T>,
        data: (Vec<String>, Vec<String>),
    ) -> Result<(), String>
    where
        T: DeserializeOwned,
    {
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }

        if self.invalid_fields(table, &data) {
            return Err(format!("Invalid column/s for db table: {}", table));
        }

        let q_string = where_string(table, data);
        self.select_query(table, q_string, objects)
    }

    ///
    /// Select all records from specified table 
    ///
    /// # Arguments
    /// * `table` - The name of a previously created table, as a string
    /// * `object` - a refrence to a generic emoty vector that will be populated with the records returned from the database
    /// 
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

    ///
    /// Deletes into from specifed table, all records that match the conditions given.
    ///
    /// # Arguments
    /// * `table` - The name of a prebiously created table, as a string
    /// * `data` - conditons to match for deleting rcords, called with the `sql` macro
    ///
    pub fn delete(&self, table: &str, data: (Vec<String>, Vec<String>)) -> Result<(), String> {
      
        if !self.tables.contains_key(table) {
            return Err(format!("DB does not contain table: {}", table));
        }

        if self.invalid_fields(table, &data) {
            return Err(format!("Invalid column/s for db table: {}", table));
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

    fn invalid_fields(&self, name: &str, data: &(Vec<String>, Vec<String>)) -> bool {
        let mut invalid = true;
        let fields = self.tables.get(name).unwrap();
        for i in 0..(data.0).len() {
            for f in fields {
                if f.0 == (data.0)[i] {
                    invalid = false
                }
            }

            if invalid {
                return true;
            }
        }
        invalid
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

/// macro that parses user options for a `sql!` command <br>
/// * Will parse tokens in the form of `field1 = value1, field2 = value2, field3 = value3`<br>
/// * Returns a tuple of string vectors, one for fields, one for values.<br>
/// * The macro will match tokens exactly and iterpret text wrapped in " as as its own token.<br>
/// * Used to parse values for `insert`, `select_where`, and `delete`.<br>
///
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

#[cfg(test)]
mod tests {
    use super::*;
    use migrate_table::MigrateTable;
    use migrate_table_derive::MigrateTable;
    use serde::Deserialize;

    #[derive(MigrateTable, Deserialize)]
    struct testModel {
        city: String,
        population: u64,
        avg_age: f64,
    }

    // we have to dispatch tests from here to prevent race conditions on the db
    // note that for these to pass you will need SQLite installed
    #[test]
    fn run_db_tests() {
        test_create_badtable();
        test_insert_valid();
        test_insert_bad();
        test_insert_badcolumn();
        test_delete_nonexistent();
        test_delete_valid();
        test_delete_badcondition();
        test_select_all_happy();
        test_select_all_badtable();
        test_select_where_happy();
        test_select_where_badtable();
        test_select_where_badcolumn();
    }

    fn setup() -> DB {
        let mut db = DB::new("rsorm_test");
        db.create_table(testModel::generate_schema()).unwrap();
        db
    }

    fn teardown() {
        std::fs::remove_file("rsorm_test").unwrap()
    }

    fn test_create_badtable() {
        #[derive(MigrateTable, Deserialize)]
        struct testBadModel {
            city: String,
            population: u8,
            avg_age: f64,
        }

        let mut db = setup();
        let result = db.create_table(testBadModel::generate_schema());
        assert!(result.is_err());
        teardown();
    }
    fn test_insert_valid() {
        let db = setup();
        let result = db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        );
        assert!(result.is_ok());

        let mut out: Vec<testModel> = Vec::new();
        db.select_all("testModel", &mut out).unwrap();
        assert_eq!(1, out.len());
        teardown();
    }

    fn test_insert_bad() {
        let db = setup();
        let result = db.insert("Model", sql!(name = "Jordan", age = 8, birthday = "idk"));
        assert!(result.is_err());
        teardown();
    }

    fn test_delete_nonexistent() {
        // this seems bad but sqlite is fine with it so it does not produce an error
        let db = setup();
        let result = db.delete(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        );
        assert!(result.is_ok());
        teardown();
    }

    fn test_insert_badcolumn() {
        let db = setup();
        let result = db.insert(
            "testModel",
            sql!(bad = "Gresham", population = 100000, avg_age = 44.3),
        );
        assert!(result.is_err());
        teardown();
    }

    fn test_delete_valid() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();

        let result = db.delete(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        );
        assert!(result.is_ok());

        let mut out: Vec<testModel> = Vec::new();
        db.select_all("testModel", &mut out).unwrap();
        assert_eq!(0, out.len());
        teardown();
    }

    fn test_delete_badcondition() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();

        let result = db.delete(
            "testModel",
            sql!(
                citymispelled = "Gresham",
                population = 100000,
                avg_age = 44.3
            ),
        );
        assert!(result.is_err());

        let mut out: Vec<testModel> = Vec::new();
        db.select_all("testModel", &mut out).unwrap();
        assert_eq!(1, out.len());
        teardown();
    }

    fn test_select_all_happy() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();
        db.insert(
            "testModel",
            sql!(city = "Sandy", population = 10000, avg_age = 62.3),
        )
        .unwrap();

        let mut out: Vec<testModel> = Vec::new();
        let result = db.select_all("testModel", &mut out);
        assert!(result.is_ok());
        assert_eq!(2, out.len());
        teardown();
    }

    fn test_select_all_badtable() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();
        db.insert(
            "testModel",
            sql!(city = "Sandy", population = 10000, avg_age = 62.3),
        )
        .unwrap();

        let mut out: Vec<testModel> = Vec::new();
        let result = db.select_all("idontexist", &mut out);
        assert!(result.is_err());
        teardown();
    }

    fn test_select_where_happy() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();
        db.insert(
            "testModel",
            sql!(city = "Sandy", population = 10000, avg_age = 62.3),
        )
        .unwrap();

        let mut out: Vec<testModel> = Vec::new();
        let result = db.select_where("testModel", &mut out, sql!(city = "Gresham"));
        assert!(result.is_ok());
        assert_eq!(1, out.len());

        for c in out {
            assert_eq!(c.city, "Gresham");
        }
        teardown();
    }

    fn test_select_where_badtable() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();
        db.insert(
            "testModel",
            sql!(city = "Sandy", population = 10000, avg_age = 62.3),
        )
        .unwrap();

        let mut out: Vec<testModel> = Vec::new();
        let result = db.select_where("idontexist", &mut out, sql!(avg_age = 62.3));
        assert!(result.is_err());
        teardown();
    }

    fn test_select_where_badcolumn() {
        let db = setup();
        db.insert(
            "testModel",
            sql!(city = "Gresham", population = 100000, avg_age = 44.3),
        )
        .unwrap();
        db.insert(
            "testModel",
            sql!(city = "Sandy", population = 10000, avg_age = 62.3),
        )
        .unwrap();

        let mut out: Vec<testModel> = Vec::new();
        let result = db.select_where("testModel", &mut out, sql!(bad = 62.3));
        assert!(result.is_err());
        teardown();
    }
}
