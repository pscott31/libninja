use openapiv3::{OpenAPI, ReferenceOr, Schema};
use ln_core::{hir, hir::Record};
use libninja::rust::mir::StructExt;

const LINK_TOKEN_CREATE: &str = include_str!("link_token_create.yaml");


fn record_for_schema(name: &str, schema: &str, spec: &OpenAPI) -> hir::Record {
    let schema = serde_yaml::from_str::<Schema>(schema).unwrap();
    let schema_ref = ReferenceOr::Item(schema);
    let mut record = ln_core::extractor::create_record(name, &schema_ref, spec);
    record.clear_docs();
    record
}


#[test]
fn test_link_token_create() {
    let mut spec = OpenAPI::default();
    spec.add_schema("UserAddress", Schema::new_object());
    spec.add_schema("UserIDNumber", Schema::new_string());
    let record = record_for_schema("LinkTokenCreateRequestUser", LINK_TOKEN_CREATE, &spec);
    let Record::Struct(struc) = record else {
        panic!("expected struct");
    };
    assert!(struc.implements_default());
}