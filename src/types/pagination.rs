use handle_error::Error;
use std::collections::HashMap;

/// Pagination struct to extract pagination parameters from query string
#[derive(Default, Debug)]
pub struct Pagination {
    /// Index of the first item to be returned
    pub limit: Option<u32>,
    /// Index of the last item to be returned
    pub offset: u32,
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
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
