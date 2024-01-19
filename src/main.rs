use biome_json_parser::JsonParserOptions;

fn main() {
    let json_str = r#""\\""#;
    let serde_result = serde_json::from_str::<String>(json_str).unwrap();
    let biome_result: String = biome_deserialize::json::deserialize_from_json_str(
        json_str,
        JsonParserOptions::default(),
        "",
    )
    .into_deserialized()
    .unwrap();

    assert_eq!(serde_result, biome_result);
}
