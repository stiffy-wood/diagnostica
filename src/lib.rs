use crate::filter::LogFilter;
use crate::zip_reader::LogFileParams;
use serde_json::json;
use std::os::raw::c_char;
use zip_reader::ZipReader;

mod encodings;
mod filter;
mod log_file;
mod zip_reader;

#[no_mangle]
pub extern "C" fn process_diagnostics(
    file_path: *const c_char,
    log_file_filters: *const *const c_char,
    log_file_filters_count: usize,
    log_line_filters: *const *const c_char,
    log_line_filters_count: usize,
) -> *mut c_char {
    let file_path = c_char_to_string(file_path);
    let log_file_filters = c_chars_to_string_vec(log_file_filters, log_file_filters_count);
    let log_line_filters = c_chars_to_string_vec(log_line_filters, log_line_filters_count);

    let log_file_filter = match LogFilter::new(log_file_filters) {
        Err(err) => return string_to_c_char(format!("Error creating log file filter: {}", err)),
        Ok(filter) => filter,
    };

    let log_line_filter = match LogFilter::new(log_line_filters) {
        Err(err) => return string_to_c_char(format!("Error creating log line filter: {}", err)),
        Ok(filter) => filter,
    };

    let log_file_params = LogFileParams {
        log_file_filter,
        log_line_filter,
        encoding_registry: match encodings::EncodingRegistry::from_json("encodings.json") {
            Err(err) => {
                return string_to_c_char(format!("Error creating encoding registry: {}", err))
            }
            Ok(registry) => registry,
        },
    };

    let zip_reader_params = zip_reader::ZipReaderParams {
        file_path,
        log_file_params,
    };
    let z_reader = ZipReader::new();

    let log_files = match zip_reader.read(&zip_reader_params) {
        Err(err) => return string_to_c_char(format!("Error reading zip file: {}", err)),
        Ok(log_files) => log_files,
    };

    let mut data = json!({
        "log_files": [],
        "errors": []
    });

    for l_f in log_files {
        match l_f {
            Err(err) => {
                if let Some(errors) = data["errors"].as_array_mut() {
                    errors.push(json!(err.to_string()))
                } else {
                    data["errors"] = json!([err.to_string()])
                }
            }
            Ok(log_file) => {
                let mut filtered_tokens: Vec<Vec<String>> = Vec::new();

                for log in log_file.filtered_logs {
                    filtered_tokens.push(log.tokens);
                }

                let js = json!({
                    "file_path": log_file.file_path,
                    "total_log_count": log_file.total_log_count,
                    "tokens": filtered_tokens
                });

                if let Some(log_files) = data["log_files"].as_array_mut() {
                    log_files.push(js);
                } else {
                    data["log_files"] = json!([js]);
                }
            }
        }
    }

    let data_string = data.to_string();
    string_to_c_char(data_string)
}

fn c_char_to_string(c_char: *const c_char) -> String {
    let c_str = unsafe { std::ffi::CStr::from_ptr(c_char) };
    let str_slice = c_str.to_str().unwrap();
    str_slice.to_string()
}

fn string_to_c_char(string: String) -> *mut c_char {
    let c_string = std::ffi::CString::new(string).unwrap();
    let c_char = c_string.into_raw();
    c_char
}

fn c_chars_to_string_vec(c_chars: *const *const c_char, count: usize) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    for i in 0..count {
        let c_char = unsafe { *c_chars.offset(i as isize) };
        let string = c_char_to_string(c_char);
        strings.push(string);
    }
    strings
}
