use std::{collections::HashSet, fs, io::Write};

use indexmap::IndexMap;
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type};

const ALLOWED_SCHEMAS: [&str; 3] = [
    "pull-request",
    "pull-request-simple",
    "pull-request-minimal",
];

const PROTECTED_NAMES: [&str; 3] = ["ref", "type", "self"];

pub fn parse() {
    let data = include_str!("../api.github.com.2022-11-28.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");

    let components = openapi
        .components
        .expect("GitHub OpenAPI spec should have components");

    let mut output = codegen::Scope::new();
    output.import("serde", "Serialize");
    output.import("serde", "Deserialize");

    let mut generated_types = HashSet::new();

    for allowed_schema_name in ALLOWED_SCHEMAS {
        let schema = match components.schemas.get(allowed_schema_name) {
            Some(ReferenceOr::Item(schema)) => schema,
            Some(ReferenceOr::Reference { reference: _ }) => {
                panic!("Base allowed schema should not be a reference")
            }
            None => panic!("Did not find allowed schema '{allowed_schema_name}' in schemas"),
        };

        let name = schema_name_as_type(allowed_schema_name);

        log::debug!("Parsing schema for {name}");

        ensure_schema_type(
            &mut output,
            &components.schemas,
            &mut generated_types,
            &name,
            schema,
        );
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

fn ensure_schema_type(
    output: &mut codegen::Scope,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    generated_types: &mut HashSet<String>,
    type_name: &str,
    schema: &Schema,
) {
    if generated_types.contains(type_name) {
        return;
    }

    let SchemaKind::Type(Type::Object(object_type)) = &schema.schema_kind else {
        return;
    };

    generated_types.insert(type_name.to_string());

    let mut struct_def = codegen::Struct::new(type_name);
    struct_def.vis("pub");
    struct_def
        .derive("Debug")
        .derive("Clone")
        .derive("PartialEq")
        .derive("Serialize")
        .derive("Deserialize");

    for (property_name, property) in &object_type.properties {
        log::debug!("Parsing property '{property_name}' of type '{type_name}'");

        struct_def.push_field(resolve_field(
            output,
            schemas,
            generated_types,
            type_name,
            property_name,
            property,
        ));
    }

    output.push_struct(struct_def);
}

fn ensure_enum_type(
    output: &mut codegen::Scope,
    generated_types: &mut HashSet<String>,
    type_name: &str,
    options: &[Option<String>],
) {
    if generated_types.contains(type_name) {
        return;
    }

    generated_types.insert(type_name.to_string());

    let mut enum_def = codegen::Enum::new(type_name);
    enum_def.vis("pub");
    enum_def
        .derive("Debug")
        .derive("Clone")
        .derive("PartialEq")
        .derive("Serialize")
        .derive("Deserialize");

    for option in options.iter().filter_map(|opt| opt.as_deref()) {
        let variant = codegen::Variant::new(schema_name_as_type(&option));

        enum_def.push_variant(variant);
    }

    output.push_enum(enum_def);
}

fn resolve_component_schema_reference<'a>(
    schemas: &'a IndexMap<String, ReferenceOr<Schema>>,
    reference: &'a str,
) -> (&'a str, &'a Schema) {
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

    (schema_name, schema)
}

fn resolve_field(
    output: &mut codegen::Scope,
    schemas: &IndexMap<String, ReferenceOr<Schema>>,
    generated_types: &mut HashSet<String>,
    parent_name: &str,
    property_name: &str,
    property: &ReferenceOr<Box<Schema>>,
) -> codegen::Field {
    let (field_name, schema): (String, &Schema) = match property {
        ReferenceOr::Reference { reference } => {
            let (schema_name, schema) = resolve_component_schema_reference(schemas, reference);

            let field_name = schema_name_as_type(schema_name);

            ensure_schema_type(output, schemas, generated_types, &field_name, schema);

            (field_name, schema)
        }
        ReferenceOr::Item(schema) => {
            let field_name = format!(
                "{}{}",
                schema_name_as_type(parent_name),
                schema_name_as_type(property_name)
            );

            ensure_schema_type(output, schemas, generated_types, &field_name, schema);

            (field_name, schema)
        }
    };

    let property_type_name = match &schema.schema_kind {
        SchemaKind::Type(schema_kind_type) => match schema_kind_type {
            Type::String(string_type) => {
                if string_type.enumeration.len() > 0 {
                    let enum_type_name = format!(
                        "{}{}",
                        schema_name_as_type(parent_name),
                        schema_name_as_type(property_name)
                    );

                    ensure_enum_type(
                        output,
                        generated_types,
                        &enum_type_name,
                        &string_type.enumeration,
                    );

                    enum_type_name
                } else {
                    "String".to_string()
                }
            }
            Type::Number(_number_type) => "f64".to_string(),
            Type::Integer(_integer_type) => "i64".to_string(),
            Type::Object(_object_type) => field_name,
            Type::Array(array_type) => {
                let Some(items_schema) = &array_type.items else {
                    panic!("Expected array to have items definition");
                };

                let (item_type_name, schema): (String, &Schema) = match items_schema {
                    ReferenceOr::Reference { reference } => {
                        let (schema_name, schema) =
                            resolve_component_schema_reference(schemas, reference);

                        let item_type_name = schema_name_as_type(schema_name);

                        ensure_schema_type(
                            output,
                            schemas,
                            generated_types,
                            &item_type_name,
                            schema,
                        );

                        (item_type_name, schema)
                    }
                    ReferenceOr::Item(schema) => {
                        let item_type_name = format!("{}Item", schema_name_as_type(property_name));

                        ensure_schema_type(
                            output,
                            schemas,
                            generated_types,
                            &item_type_name,
                            schema,
                        );

                        (item_type_name, schema)
                    }
                };

                let vec_item_type_name = match &schema.schema_kind {
                    SchemaKind::Type(schema_kind_type) => match schema_kind_type {
                        Type::String(_string_type) => "String".to_string(),
                        Type::Number(_number_type) => "f64".to_string(),
                        Type::Integer(_integer_type) => "i64".to_string(),
                        Type::Object(_object_type) => item_type_name,
                        Type::Array(_array_type) => todo!(),
                        Type::Boolean(_boolean_type) => "bool".to_string(),
                    },
                    _ => todo!(),
                };

                let vec_item_type_name = if schema.schema_data.nullable {
                    format!("Option<{vec_item_type_name}>")
                } else {
                    vec_item_type_name
                };

                format!("Vec<{vec_item_type_name}>")
            }
            Type::Boolean(_boolean_type) => "bool".to_string(),
        },
        _ => todo!(),
    };

    let property_type_name = if schema.schema_data.nullable {
        format!("Option<{property_type_name}>")
    } else {
        property_type_name
    };

    let sanitized_property_name = sanitize_field_name(property_name);
    let mut field = codegen::Field::new(
        sanitized_property_name.as_deref().unwrap_or(property_name),
        codegen::Type::new(property_type_name),
    );

    if sanitized_property_name.is_some() {
        field.annotation(format!(r#"#[serde(rename = "{property_name}")]"#));
    }

    if let Some(description) = &schema.schema_data.description {
        field.doc(description);
    }

    field
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

fn sanitize_field_name(field_name: &str) -> Option<String> {
    if PROTECTED_NAMES.contains(&field_name) {
        Some(format!("{field_name}_"))
    } else {
        None
    }
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
