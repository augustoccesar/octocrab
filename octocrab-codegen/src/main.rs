use std::{fs, io::Write};

use indexmap::IndexMap;
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type};

const ALLOWED_SCHEMAS: [&str; 3] = [
    "pull-request",
    "pull-request-simple",
    "pull-request-minimal",
];

pub fn parse() {
    let data = include_str!("../api.github.com.2022-11-28.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");

    let components = openapi
        .components
        .expect("GitHub OpenAPI spec should have components");
    let mut schemas = Vec::new();

    for allowed_schema_name in ALLOWED_SCHEMAS {
        match components.schemas.get(allowed_schema_name) {
            Some(ReferenceOr::Item(schema)) => {
                if schema.schema_data.title.is_none() {
                    panic!("Expect schema to have a title");
                }

                schemas.push(schema)
            }
            Some(ReferenceOr::Reference { reference: _ }) => {
                panic!("Base allowed schema should not be a reference")
            }
            None => panic!("Did not find allowed schema '{allowed_schema_name}' in schemas"),
        }
    }

    let mut output = codegen::Scope::new();

    for schema in &schemas {
        let name: String = schema
            .schema_data
            .title
            .as_ref()
            .unwrap()
            .split(" ")
            .collect();

        log::debug!("Parsing schema for {name}");

        match &schema.schema_kind {
            SchemaKind::Type(ty) => match ty {
                Type::Object(object_type) => {
                    let mut struct_def = codegen::Struct::new(&name);

                    for (property_name, property) in &object_type.properties {
                        log::debug!("Parsing property '{property_name}'");

                        struct_def.push_field(resolve_field(
                            &mut output,
                            &components.schemas,
                            &name,
                            &property_name,
                            &property,
                        ));
                    }

                    output.push_struct(struct_def);
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    // TODO(@augustoccesar)[2026-05-27]: Have the target be configurable
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("../octocrab-types/src/generated.rs")
        .unwrap();

    file.write_all(output.to_string().as_bytes()).unwrap();
}

fn resolve_field(
    output: &mut codegen::Scope,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    parent_name: &str,
    property_name: &str,
    property: &ReferenceOr<Box<Schema>>,
) -> codegen::Field {
    let (field_name, schema): (String, &Schema) = match property {
        ReferenceOr::Reference { reference } => {
            if !reference.starts_with("#/components/schemas/") {
                panic!("Expected field reference to start with #/components/schemas/");
            }

            let schema_name = reference
                .split("#/components/schemas/")
                .nth(1)
                .expect("Schema reference should follow #/components/schemas/{name} format");

            log::debug!("Looking up for schema named '{schema_name}' on the spec");

            let schema = match schemas
                .get(schema_name)
                .expect("Referenced schema should exist on the spec")
            {
                ReferenceOr::Reference { reference: _ } => todo!("Reference of reference"),
                ReferenceOr::Item(schema) => schema,
            };

            (schema_name_as_type(schema_name), schema)
        }
        ReferenceOr::Item(schema) => (
            format!(
                "{}{}",
                schema_name_as_type(parent_name),
                schema_name_as_type(property_name)
            ),
            schema,
        ),
    };

    let mut property_type_name = String::new();

    if schema.schema_data.nullable {
        property_type_name.push_str("Option<");
    }

    match &schema.schema_kind {
        SchemaKind::Type(schema_kind_type) => match schema_kind_type {
            Type::String(string_type) => property_type_name.push_str("String"),
            Type::Number(number_type) => todo!(),
            Type::Integer(integer_type) => property_type_name.push_str("i64"),
            Type::Object(object_type) => property_type_name.push_str(&field_name),
            Type::Array(array_type) => {
                let Some(items_schema) = &array_type.items else {
                    panic!("Expected array to have items definition");
                };

                let (item_type_name, schema): (String, &Schema) = match items_schema {
                    ReferenceOr::Reference { reference } => {
                        if !reference.starts_with("#/components/schemas/") {
                            panic!("Expected field reference to start with #/components/schemas/");
                        }

                        let schema_name = reference.split("#/components/schemas/").nth(1).expect(
                            "Schema reference should follow #/components/schemas/{name} format",
                        );

                        log::debug!("Looking up for schema named '{schema_name}' on the spec");

                        let schema = match schemas
                            .get(schema_name)
                            .expect("Referenced schema should exist on the spec")
                        {
                            ReferenceOr::Reference { reference: _ } => {
                                todo!("Reference of reference")
                            }
                            ReferenceOr::Item(schema) => schema,
                        };

                        (schema_name_as_type(schema_name), schema)
                    }
                    ReferenceOr::Item(schema) => (
                        format!("{}Item", schema_name_as_type(property_name)),
                        schema,
                    ),
                };

                let mut vec_item_type_name = String::new();

                if schema.schema_data.nullable {
                    vec_item_type_name.push_str("Option<");
                }

                vec_item_type_name.push_str(&item_type_name);

                if schema.schema_data.nullable {
                    vec_item_type_name.push_str(">");
                }

                property_type_name.push_str(&format!("Vec<{vec_item_type_name}>"));
            }
            Type::Boolean(boolean_type) => property_type_name.push_str("bool"),
        },
        _ => todo!(),
    }

    if schema.schema_data.nullable {
        property_type_name.push('>');
    }

    codegen::Field::new(property_name, codegen::Type::new(property_type_name))
}

fn schema_name_as_type(schema_name: &str) -> String {
    schema_name
        .split(['-', '_'])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first_char) => first_char.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        })
        .collect()
}

fn main() {
    env_logger::init();

    parse();
}

#[cfg(test)]
mod tests {
    use crate::schema_name_as_type;

    #[test]
    fn schema_name_as_type_works() {
        assert_eq!("SomeName", schema_name_as_type("some-name"));
    }
}
