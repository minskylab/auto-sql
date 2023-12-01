// use commons::introspective::Introspective;
use darling::FromDeriveInput;
use proc_macro::TokenStream;

use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput, Default)]
#[darling(default, attributes(auto_sql))]
struct Opts {
    relation: Option<String>,
    client: Option<Ident>,
}

#[proc_macro_derive(AutoSQL, attributes(auto_sql))]
pub fn auto_sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // let a = Introspective::introspect();

    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    let client_name = opts
        .client
        .unwrap_or(syn::Ident::new("Client", proc_macro2::Span::call_site()));

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs with named fields are supported"),
    };

    let type_name = input.ident;

    let lower_singular = type_name.to_string().to_lowercase();
    let lower_plural = format!("{}s", lower_singular);

    let title_singular = lower_singular.capitalize();
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

    let get_method_name = format_ident!("get_{}", lower_singular);

    let get_list_method_name = format_ident!("get_{}", lower_plural);
    let get_list_input_name = format_ident!("Get{}Input", title_plural);
    let get_list_where_name = format_ident!("Get{}Where", title_plural);

    let get_list_where_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #[builder(setter(strip_option), default)]
            pub #field_name: Option<#field_type>
        }
    });

    let update_method_name = format_ident!("update_{}", lower_singular);
    let update_input_name = format_ident!("Update{}Input", title_singular);

    let update_input_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            #[builder(setter(strip_option), default)]
            pub #field_name: Option<#field_type>
        }
    });

    let delete_method_name = format_ident!("delete_{}", lower_singular);

    let sql_table_fields = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let sql_type = syn_type_to_sql_type(field_type);

            format!("{} {}", field_name.as_ref().unwrap(), sql_type)
        })
        .collect::<Vec<String>>()
        .join(", ");

    let sql_table = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        type_name, sql_table_fields
    );

    let token_stream = quote! {
        #[async_trait::async_trait]
        pub trait #crud_operations_trait_name {
            async fn #create_method_name(&self, input: #create_input_name) -> Result<#type_name, Box<dyn std::error::Error>>;
            async fn #get_method_name(&self, id: uuid::Uuid) -> Result<#type_name, Box<dyn std::error::Error>>;
            async fn #get_list_method_name(&self, input: Option<#get_list_input_name>) -> Result<Vec<#type_name>, Box<dyn std::error::Error>>;
            async fn #update_method_name(&self, id: uuid::Uuid, input: #update_input_name) -> Result<#type_name, Box<dyn std::error::Error>>;
            async fn #delete_method_name(&self, id: uuid::Uuid) -> Result<#type_name, Box<dyn std::error::Error>>;
        }

        #[derive(Default, derive_builder::Builder)]
        #[builder(pattern = "owned")]
        pub struct #create_input_name {
            #(#create_input_fields),*
        }

        #[derive(Default, derive_builder::Builder)]
        #[builder(pattern = "owned")]
        pub struct #get_list_input_name {
            #[builder(setter(strip_option), default)]
            pub filter: Option<#get_list_where_name>,

            #[builder(setter(strip_option), default)]
            pub sort_by: Option<String>,
            // #[builder(setter(strip_option), default)]
            // pub sort_order: Option<SortOrder>,

            #[builder(setter(into, strip_option), default = "Some(100)")]
            pub limit: Option<i32>,
            #[builder(setter(into, strip_option), default = "Some(0)")]
            pub offset: Option<i32>,
        }

        #[derive(Default, derive_builder::Builder)]
        #[builder(pattern = "owned")]
        pub struct #get_list_where_name {
            #(#get_list_where_fields),*
        }

        #[derive(Default, derive_builder::Builder)]
        #[builder(pattern = "owned")]
        pub struct #update_input_name {
            #(#update_input_fields),*
        }

        #[async_trait::async_trait]
        impl #crud_operations_trait_name for #client_name {
            async fn #create_method_name(&self, input: #create_input_name) -> Result<#type_name, Box<dyn std::error::Error>> {
                todo!()
            }

            async fn #get_method_name(&self, id: uuid::Uuid) -> Result<#type_name, Box<dyn std::error::Error>> {
                todo!()
            }

            async fn #get_list_method_name(&self, input: Option<#get_list_input_name>) -> Result<Vec<#type_name>, Box<dyn std::error::Error>> {
                todo!()
            }

            async fn #update_method_name(&self, id: uuid::Uuid, input: #update_input_name) -> Result<#type_name, Box<dyn std::error::Error>> {
                todo!()
            }

            async fn #delete_method_name(&self, id: uuid::Uuid) -> Result<#type_name, Box<dyn std::error::Error>> {
                todo!()
            }
        }

        impl #type_name {
            pub async fn digest<C:  auto_sql::commons::Introspective>(&self, client: &C) {
                client.introspect().await.unwrap();
            }
        }

        impl auto_sql::commons::AsSQLArtifacts for #type_name {
            fn as_sql_artifacts() -> Vec<auto_sql::commons::SQLArtifact> {
                vec![
                    auto_sql::commons::SQLArtifact {
                        kind: auto_sql::commons::SQLArtifactKind::Table,
                        name: stringify!(#type_name).to_string(),
                        sql: stringify!(#sql_table).to_string(),
                    }
                ]
            }
        }
    };

    let expanded = token_stream;

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

trait Capitalizer {
    fn capitalize(&self) -> String;
}

impl Capitalizer for String {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

fn syn_type_to_sql_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => {
            let segment = segments.first().unwrap();
            let ident = &segment.ident;

            match ident.to_string().as_str() {
                "String" => "TEXT".to_string(),
                "i32" => "INTEGER".to_string(),
                "bool" => "BOOLEAN".to_string(),
                "f64" => "DOUBLE PRECISION".to_string(),
                "DateTime<Utc>" => "TIMESTAMP WITH TIME ZONE".to_string(),
                _ => panic!("Unsupported type"),
            }
        }
        _ => panic!("Unsupported type"),
    }
}
