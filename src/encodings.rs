use std::fs;
use std::io::Error;
use encoding_rs::Encoding;
use serde_json;

pub struct EncodingRegistry{
    encodings: Vec<&'static Encoding>
}

impl EncodingRegistry{
    pub fn from_json(file_path: &str) -> Result<EncodingRegistry, Error>{
        let json_data = match fs::read_to_string(file_path) {
            Err(err) => return Err(err),
            Ok(str) => str
        };

        let encoding_names: Vec<String> = match serde_json::from_str(&json_data){
            Err(err) => return Err(Error::from(err)),
            Ok(enc) => enc
        };

        let mut encodings = Vec::new();
        for encoding_name in encoding_names{
            let encoding = match Encoding::for_label(encoding_name.as_bytes()){
                None => continue,
                Some(enc) => enc
            };
            encodings.push(encoding);
        }

        return Ok(EncodingRegistry{
            encodings
        })
    }

    pub fn decode(&self, buffer: &[u8]) -> Option<String>{
        for encoding in &self.encodings {
            let (result, _, malformed) = encoding.decode(buffer);
            if !result.is_empty() && !malformed {
                return Some(result.into_owned());
            }
        }
        None
    }
}
