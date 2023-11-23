// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use darling::FromDeriveInput;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput, Default)]
#[darling(default, attributes(auto_sql))]
struct Opts {
    relation: Option<String>,
    // prefix: Option<String>,
}

#[proc_macro_derive(AutoSQL, attributes(auto_sql))]
pub fn auto_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    println!("{:?}", opts);

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs with named fields are supported"),
    };

    let ty_name = input.ident;

    let lower_singular = ty_name.to_string().to_lowercase();
    let lower_plural = format!("{}s", lower_singular);

    let title_singular = capitalize(lower_singular.as_str());
    let title_plural = format!("{}s", title_singular);

    let crud_operations_trait_name = format_ident!("{}CrudOperations", title_singular);

    let create_method_name = format_ident!("create_{}", lower_singular);
    let create_input_name = format_ident!("Create{}Input", title_singular);

    let create_input_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #[builder(setter(strip_option), default)]
            pub #field_name: #field_type
        }
    });

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #[async_trait::async_trait]
        pub trait #crud_operations_trait_name {
            async fn #create_method_name(&self, input: #create_input_name) -> Result<#ty_name, Box<dyn std::error::Error>>;
            // async fn get_{#lower_singular}(&self, id: Uuid) -> Result<{#title_singular}, Box<dyn Error>>;
            // async fn get_{#lower_plural}(&self, input: Option<Get{#title_plural}Input>)
            //     -> Result<Vec<{#title_singular}>, Box<dyn Error>>;
            // async fn update_{#lower_singular}(
            //     &self,
            //     id: Uuid,
            //     input: Update{#title_singular}Input,
            // ) -> Result<{#title_singular}, Box<dyn Error>>;
            // async fn delete_{#lower_singular}(&self, id: Uuid) -> Result<{#title_singular}, Box<dyn Error>>;
        }

        #[derive(Default, derive_builder::Builder)]
        #[builder(pattern = "owned")]
        pub struct #create_input_name {
            #(#create_input_fields),*
            // #[builder(setter(strip_option), default)]
            // pub description: Option<String>,
            // #[builder(setter(strip_option), default)]
            // pub due_date: Option<chrono::DateTime<chrono::Utc>>,
            // #[builder(setter(strip_option), default)]
            // pub project_id: Option<uuid::Uuid>,
            // #[builder(setter(strip_option), default)]
            // pub lead_id: Option<uuid::Uuid>,
            // #[builder(setter(strip_option), default)]
            // pub parent_id: Option<uuid::Uuid>,
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
