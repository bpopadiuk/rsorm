extern crate proc_macro;

use crate::proc_macro::TokenStream;
use syn;
use quote::quote;

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
            fn migrate_table() {
                println!("STRUCT NAME: {}\nFIELDS: {}", stringify!(#name), stringify!(#fields));
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
