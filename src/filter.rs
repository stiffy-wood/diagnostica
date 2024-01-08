
use regex::{Regex, Error};

pub struct LogFilter {
    filters: Vec<Regex>
}

impl LogFilter {
    pub fn new(filters: Vec<String>) -> Result<LogFilter, Error>{
        let mut regex_filters: Vec<Regex> = Vec::new();
        for filter in filters{
            let regex = match Regex::new(&filter){
                Err(err) => return Err(err),
                Ok(regex) => regex
            };
            regex_filters.push(regex);
        }

        Ok(LogFilter {
            filters: regex_filters
        })
    }

    pub fn matches(&self, text: &str) -> bool{
        for filter in &self.filters{
            if !filter.is_match(text){
                return false;
            }
        }
        return true;
    }
}
