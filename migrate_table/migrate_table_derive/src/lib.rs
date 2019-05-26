#![recursion_limit = "1024"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MigrateTable)]
pub fn migrate_table_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_migrate_table(&ast)
}

fn impl_migrate_table(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let fields = field_names(data).expect("ERROR: rsorm can only migrate structs");
    let gen = quote! {
        impl MigrateTable for #name {
            fn generate_schema() -> (String, Vec<(String, String)>) {
                let name = String::from(stringify!(#name));
                let field_str = stringify!(#fields);
                let fs = field_str.split(" ").collect::<Vec<&str>>();
                let mut field_tups = Vec::new();
                for i in (3..fs.len()).step_by(4) {
                    field_tups.push((String::from(fs[i-2]), String::from(fs[i])));
                }
                (name, field_tups)
            }
        }
    };
    gen.into()
}

fn field_names(data: &syn::Data) -> Result<syn::Fields, &'static str> {
    match data {
        syn::Data::Struct(data) => {
            return Ok(data.fields.clone());
        }
        _ => {
            return Err("invalid type");
        }
    }
}
