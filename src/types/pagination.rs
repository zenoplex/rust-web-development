use crate::error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseFailed)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(error::Error::ParseFailed)?;

        return Ok(Pagination { start, end });
    }

    Err(error::Error::MissingParameters)
}
