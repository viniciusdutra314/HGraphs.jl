use std::io::Read;

enum HifIoError {
    InvalidJson,
}

// pub fn read_hif<R: Read>(reader: R) -> Result<(), HifIoError> {
//     let json: String = serde_json::from_reader(reader).unwrap();
//     jsonschema::validate(schema, instance)
// }
