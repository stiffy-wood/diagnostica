use std::fs;
use std::io::Error;
use zip::ZipArchive;

use crate::encodings::EncodingRegistry;
use crate::filter::LogFilter;
use crate::log_file::LogFile;


pub struct ZipReaderParams {
    pub file_path: String,
    pub log_file_params: LogFileParams
}

pub struct LogFileParams{
    pub log_file_filter: LogFilter,
    pub log_line_filter: LogFilter,
    pub encoding_registry: EncodingRegistry
}

pub struct ZipReader{
}

impl ZipReader{
    pub fn new() -> Self{
        ZipReader{
        }
    }

    pub fn read(&self, info: &ZipReaderParams) -> Result<Vec<Result<LogFile, Error>>, Error>{
        let mut archive = match self.open_zip(&info.file_path){
            Err(err) => return Err(err),
            Ok(archive) => archive
        };

        let mut log_files: Vec<Result<LogFile, Error>> = Vec::new();

        for i in 0..archive.len(){
            let file = match archive.by_index(i){
                Err(err) => {
                    log_files.push(Err(Error::from(err)));
                    continue;
                },
                Ok(file) => file
            };

            let log_file = match LogFile::from_zip_file(info.file_path.to_string(), file, &info.log_file_params){
                None => continue,
                Some(log_file) => log_file
            };

            log_files.push(log_file);
        }

        Ok(log_files)
    }

    fn open_zip(&self, file_path: &str) -> Result<ZipArchive<fs::File>, Error>{
        let file = match fs::File::open(&file_path){
            Err(err) => return Err(err),
            Ok(file) => file
        };

        let archive = match ZipArchive::new(file){
            Err(err) => return Err(Error::from(err)),
            Ok(archive) => archive
        };

        Ok(archive)
    }
}