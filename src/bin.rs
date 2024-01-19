mod encodings;
mod filter;
mod log_file;
mod zip_reader;

use std::{env, fs, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let file_path = find_zip()?;
    
    println!("{:?}", file_path);
    
    let log_file_filter = filter::LogFilter::new(vec!["OLM".to_string()])?;
    let log_line_filter = filter::LogFilter::new(vec!["ERROR".to_string()])?;

    let log_file_params = zip_reader::LogFileParams {
        log_file_filter,
        log_line_filter,
        encoding_registry: encodings::EncodingRegistry::from_json("src/encodings.json")?};

    let zip_reader_params = zip_reader::ZipReaderParams {
        log_file_params,
        file_path,
    };

    println!("Reading zip file");

    let z_reader = zip_reader::ZipReader::new();

    let results = z_reader.read(&zip_reader_params)?;

    for log_file in results {
       match log_file {
           Ok(lf) => println!("{:?}", lf.file_path),
           Err(e) => println!("{:?}", e),
       }
    }

    Ok(())
}

fn find_zip() -> Result<String, Box<dyn Error>> {
    let cur_dir = env::current_dir()?;

    for entry in fs::read_dir(cur_dir)? {
        let path = match &entry {
            Ok(p) => p.path(),
            Err(_) => continue,
        };
        if path.extension().unwrap_or_default() == "zip"{
            return Ok(path.to_str().unwrap().to_string());
        }
    }
    Err("No zip file found".into())
}
