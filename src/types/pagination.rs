use handle_error::Error;
use std::collections::HashMap;

/// Pagination struct to extract pagination parameters from query string
#[derive(Debug)]
pub struct Pagination {
    /// Index of the first item to be returned
    pub start: usize,
    /// Index of the last item to be returned
    pub end: usize,
}

/// Extract pagination parameters from /questions endpoint
/// # Example query
/// GET request to following endpoint
/// `/questions?start=0&end=10`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "0".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 0);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseFailed)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseFailed)?;

        return Ok(Pagination { start, end });
    }

    Err(Error::MissingParameters)
}
