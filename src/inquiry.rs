use std::collections::HashMap;
use inquire::{Select, Text};
use crate::error::Error;

pub fn select(args: Vec<&str>, question: &str) -> Result<String, Error> {
    let res = Select::new(question, args)
        .prompt()?;

    Ok(res.to_owned())
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
