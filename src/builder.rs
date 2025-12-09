use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Publisher {
    pub publisher_id: i64,
    pub publisher_name: String,
}

pub trait NamedResource {
    fn id(&self) -> i64;
    fn name(&self) -> &str;
}

impl NamedResource for Publisher {
    fn id(&self) -> i64 {
        self.publisher_id
    }

    fn name(&self) -> &str {
        &self.publisher_name
    }
}

pub fn build_name_id_map<T>(items: Vec<T>) -> HashMap<String, i64>
where
    T: NamedResource,
{
    let mut map = HashMap::new();

    for item in items {
        map.insert(item.name().to_string(), item.id());
    }

    map
}
