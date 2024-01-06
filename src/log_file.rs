use crate::encodings::EncodingRegistry;
use crate::filter::LogFilter;
use crate::zip_reader::LogFileParams;
use std::io::Error;
use zip::read::ZipFile;

pub struct Log {
    msg: String,
    pub tokens: Vec<String>,
}

impl Log {
    pub fn new(msg: String) -> Log {
        Log {
            msg,
            tokens: Vec::new(),
        }
    }
    pub fn tokenize(&mut self) {
        self.tokens = self
            .msg
            .split(|x| x == ' ')
            .map(|s| s.to_string())
            .collect()
    }
}

pub struct LogFile {
    pub file_path: String,
    pub filtered_logs: Vec<Log>,
    pub total_log_count: u32,
}

impl LogFile {
    pub fn new(path: String) -> LogFile {
        LogFile {
            file_path: path,
            filtered_logs: Vec::new(),
            total_log_count: 0,
        }
    }

    pub fn from_zip_file(
        path: String,
        file: ZipFile,
        params: &LogFileParams,
    ) -> Option<Result<LogFile, Error>> {
        let mut log_file = LogFile::new(path);

        if !params.log_file_filter.matches(file.name()) {
            return None;
        }

        let decoded = match log_file.read(file, &params.encoding_registry) {
            Err(err) => return Some(Err(err)),
            Ok(decoded) => decoded,
        };

        log_file.filtered_logs = match log_file.split_logs(decoded, &params.log_line_filter) {
            Err(err) => return Some(Err(err)),
            Ok(logs) => logs,
        };

        Some(Ok(log_file))
    }

    fn read(&self, mut file: ZipFile, registry: &EncodingRegistry) -> Result<String, Error> {
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Err(err) => return Err(err),
            Ok(_) => (),
        };

        match registry.decode(&buffer) {
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Could not decode file",
                ))
            }
            Some(decoded) => Ok(decoded),
        }
    }

    fn split_logs(&mut self, text: String, log_filter: &LogFilter) -> Result<Vec<Log>, Error> {
        let mut logs: Vec<Log> = Vec::new();
        for line in text.lines() {
            self.total_log_count += 1;
            if !log_filter.matches(line) {
                continue;
            }

            let mut log = Log::new(line.to_string());
            log.tokenize();
            logs.push(log);
        }
        Ok(logs)
    }
}
