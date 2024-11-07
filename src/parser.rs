use crate::data::ShoppingItem;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse shopping item")]
    InvalidFormat,
}
//
pub fn parse_shopping_item(line: &str) -> Result<ShoppingItem, ParseError> {
    let re = Regex::new(r"^(?P<item>\w+)\s+(?P<quantity>\d+(\.\d+)?)\s+(?P<unit>\w+)\s+-\s+(?P<category>\w+)$")
        .map_err(|_| ParseError::InvalidFormat)?;

    if let Some(caps) = re.captures(line) {
        let item = caps["item"].to_string();
        let quantity = caps["quantity"].parse::<f32>().map_err(|_| ParseError::InvalidFormat)?;
        let unit = caps["unit"].to_string();
        let category = caps["category"].to_string();

        Ok(ShoppingItem { item, quantity, unit, category })
    } else {
        Err(ParseError::InvalidFormat)
    }
}
