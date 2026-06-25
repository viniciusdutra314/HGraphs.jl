// use hypergraph::*;
// use std::io::Read;
// use std::sync::LazyLock;

// enum HifIoError {
//     InvalidJson,
//     WrongSchema,
// }
// const HIF_VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
//     let HIF_schema = serde_json::from_str(include_str!(
//         "../../assets/HIF-standard/schemas/hif_schema_v0.1.0.json"
//     ))
//     .unwrap();
//     jsonschema::validator_for(&HIF_schema).unwrap()
// });

// pub fn read_hif<R: Read>(reader: R) -> Result<Hypergraph<>, HifIoError> {
//     let json: serde_json::Value =
//         serde_json::from_reader(reader).map_err(|_| HifIoError::InvalidJson)?;
//     HIF_VALIDATOR
//         .validate(&json)
//         .map_err(|_| HifIoError::WrongSchema)?;

// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_hif_compliant() {
//         for path in std::fs::read_dir("assets/HIF-standard/tests/test_files/HIF-compliant").unwrap()
//         {
//             let json_file = std::fs::File::open(path.unwrap().path()).unwrap();
//             assert!(read_hif(json_file).is_ok());
//         }
//     }
//     #[test]
//     fn test_hif_non_compliant() {
//         for path in
//             std::fs::read_dir("assets/HIF-standard/tests/test_files/HIF-non-compliant").unwrap()
//         {
//             let json_file = std::fs::File::open(path.unwrap().path()).unwrap();
//             assert!(read_hif(json_file).is_err());
//         }
//     }
// }
