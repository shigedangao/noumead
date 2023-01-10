use std::collections::HashMap;
use inquire::{Select, Text, MultiSelect};
use crate::error::{Error, self};

// constant
const SELECT_PAGE_SIZE: usize = 20;

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
        .with_page_size(SELECT_PAGE_SIZE)
        .prompt()?;

    let idx = items.binary_search(&res)
        .map_err(|_| Error::ScenarioErr(error::SELECTED_ITEM_NOT_FOUND_ERR.to_string()))?;

    Ok((res, idx))
}

/// Display a list of items where the user can select multiple options
///
/// # Arguments
///
/// * `args` - &[T]
/// * `question` - &str
pub fn multi_select<T: ToString>(args: &[T], question: &str) -> Result<(Vec<String>, Vec<usize>), Error> {
    let items: Vec<String> = args.iter()
        .map(|arg| arg.to_string())
        .collect();

    let res = MultiSelect::new(question, items.clone())
        .with_page_size(SELECT_PAGE_SIZE)
        .prompt()?;

    let indexes: Vec<usize> = res
        .iter()
        .filter_map(|v| items.binary_search(v).ok())
        .collect();

    Ok((res, indexes))
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
            return Err(Error::ScenarioErr(error::MISSING_REQUIRED_FIELD_ERR.to_string()))
        }

        map.insert(item, answer);
    }

    Ok(map)
}
