use migrate_table::MigrateTable;
use migrate_table_derive::MigrateTable;

#[allow(dead_code)]
#[derive(MigrateTable)]
struct Model{
    name: String,
    age: u64,
    birthday: f64,
}

 pub fn create_table(fields: &Vec<(String, String)>) -> Result<(), &'static str> {
    let legal_types: std::collections::HashSet<String> = vec!("String".to_string(), "u64".to_string(), "f64".to_string()).into_iter().collect();
    for f in fields {
        if !legal_types.contains(&f.1) {
            return Err("RSORM models can only contain the following types: u64, f64, and String")
        } 
    }

    // TODO: this function should now use the name and fields arguments to make a SQL call to create a table
    Ok(())
}

fn main() {
    let (name, fields) = Model::generate_schema();
    create_table(&fields).unwrap();
    println!("NAME: {:?}\nFIELDS: {:?}", name, fields)
}
