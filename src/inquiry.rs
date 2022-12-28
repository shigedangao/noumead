use std::collections::HashMap;
use inquire::{Select, Text};
use crate::error::Error;

// constant
const SELECTED_ITEM_NOT_FOUND: &str = "Unable to found the selected item";

pub trait ItemName {
    fn get_name(&self) -> &str;
}

pub fn select<T: ItemName>(args: &Vec<T>, question: &str) -> Result<(String, usize), Error> {
    let items: Vec<&str> = args.iter()
        .map(|i| i.get_name())
        .collect();

    let res = Select::new(question, items.clone())
        .prompt()?;

    let idx = items.binary_search(&res)
        .map_err(|_| Error::ScenarioErr(SELECTED_ITEM_NOT_FOUND.to_string()))?;

    Ok((res.to_owned(), idx))
}

pub fn prompt_vector(items: Vec<String>, msg: &str) -> Result<HashMap<String, String>, Error> {
    let mut map = HashMap::new();
    for item in items {
        let answer = Text::new(&format!("{}: {}", msg, item))
            .prompt()?;

        map.insert(item, answer);
    }

    Ok(map)
}
