use convert_case::{Case, Casing};
use quote::{__private::TokenStream, format_ident, quote};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ast::{dmmf, AST};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<'a> {
    pub generator: Generator,
    pub schema_path: String,
    pub dmmf: dmmf::Document,
    pub datamodel: String,
    #[serde(skip)]
    pub ast: Option<AST<'a>>
}

fn default_package() -> String {
    "./db".into()
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "default_package")]
    pub package: String,
    // pub output: String,
    pub disable_gitignores: Option<String>,
    pub disable_rust_binaries: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub output: Value,
    pub name: String,
    pub provider: Value,
    pub config: Config,
    pub binary_targets: Vec<Value>,
    // pub pinned_binary_target: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub from_env_var: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    MySQL,
    Mongo,
    SQLite,
    PostgreSQL,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub name: String,
    pub active_provider: ProviderType,
    pub provider: String,
    pub url: EnvValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnvValue {
    pub from_env_var: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BinaryPaths {
    pub migration_engine: HashMap<String, String>,
    pub query_engine: HashMap<String, String>,
    pub introspection_engine: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLType(pub String);

impl GraphQLType {
    pub fn string(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> String {
        self.tokens().to_string()
    }

    pub fn tokens(&self) -> TokenStream {
        let string = self.string();

        match string {
            "Int" => quote!(i32),
            "BigInt" => quote!(i64),
            "Float" => quote!(f64),
            "Decimal" => quote!(f64),
            "Boolean" => quote!(bool),
            "Bytes" => quote!(Vec<u8>),
            "DateTime" => quote!(chrono::DateTime<chrono::Utc>),
            "Json" => quote!(serde_json::Value),
            _ => {
                let ident = format_ident!("{}", string.to_case(Case::Pascal));
                quote!(#ident)
            }
        }
    }
}
