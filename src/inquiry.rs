use std::collections::HashMap;
use inquire::{Select, Text};
use crate::error::Error;

// constant
const SELECTED_ITEM_NOT_FOUND: &str = "Unable to found the selected item";

/// Display a list of items to the user
///
/// # Arguments
///
/// * `args` - &Vec<T: ToString>
/// * `question` - &str
pub fn select<T: ToString>(args: &Vec<T>, question: &str) -> Result<(String, usize), Error> {
    let items: Vec<String> = args.into_iter()
        .map(|i| i.to_string())
        .collect();

    let res = Select::new(question, items.clone())
        .prompt()?;

    let idx = items.binary_search(&res)
        .map_err(|_| Error::ScenarioErr(SELECTED_ITEM_NOT_FOUND.to_string()))?;

    Ok((res.to_owned(), idx))
}

/// Show a prompt from multiple options the user need may (or not) answer
///
/// # Arguments
///
/// * `items` - Vec<String>
/// * `msg` - &str
pub fn prompt_vector(items: Vec<String>, msg: &str) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    for item in items {
        let answer = Text::new(&format!("{}: {}", msg, item))
            .prompt()?;

        map.insert(item, answer);
    }

    Ok(map)
}
