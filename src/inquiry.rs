use std::collections::HashMap;
use inquire::{Select, Text};
use crate::error::Error;

// constant
const SELECTED_ITEM_NOT_FOUND_ERR: &str = "Unable to found the selected item";
const MISSING_REQUIRED_FIELD_ERR: &str = "You must fill this field as the value is required";

/// Display a list of items to the user
///
/// # Arguments
///
/// * `args` - &[T: ToString]
/// * `question` - &str
pub fn select<T: ToString>(args: &[T], question: &str) -> Result<(String, usize), Error> {
    let items: Vec<String> = args.iter()
        .map(|i| i.to_string())
        .collect();

    let res = Select::new(question, items.clone())
        .prompt()?;

    let idx = items.binary_search(&res)
        .map_err(|_| Error::ScenarioErr(SELECTED_ITEM_NOT_FOUND_ERR.to_string()))?;

    Ok((res, idx))
}

/// Show a prompt from multiple options the user need may (or not) answer
///
/// # Arguments
///
/// * `items` - Vec<String>
/// * `msg` - &str
/// * `required` - bool
pub fn prompt_vector(items: Vec<String>, msg: &str, required: bool) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    for item in items {
        let answer = Text::new(&format!("{}: {}", msg, item))
            .prompt()?;

        if answer.is_empty() && required {
            return Err(Error::ScenarioErr(MISSING_REQUIRED_FIELD_ERR.to_string()))
        }

        map.insert(item, answer);
    }

    Ok(map)
}
