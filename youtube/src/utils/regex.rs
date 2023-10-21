use regex::Regex;

use crate::prelude::AudiophileError;

pub fn fullmatch(haystack: &str, needle: &str) -> Result<bool, AudiophileError> {
    let regex = Regex::new(needle).map_err(|e| AudiophileError {
        message: "Something went wrong creating the regex instance",
        location: "internal REGEX::FULLMATCH",
        cause: Some(Box::new(e)),
    })?;
    let text_match = match regex.find(haystack) {
        Some(m) => m,
        None => return Ok(false),
    };
    Ok(text_match.start() == 0 && text_match.end() == needle.len())
}
