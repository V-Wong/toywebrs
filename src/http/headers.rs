use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        self.0.insert(key.to_string(), value.to_string());
        self
    }
}

impl From<HashMap<String, String>> for Headers {
    fn from(value: HashMap<String, String>) -> Self {
        Self(value)
    }
}

impl FromIterator<String> for Headers {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Headers(
            iter.into_iter()
                .flat_map(|header| {
                    let (key, value) = header.split_once(":")?;
                    Some((key.to_string(), value.trim_start().to_string()))
                })
                .collect(),
        )
    }
}

impl ToString for Headers {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("{key}: {value}\r\n"))
            .collect::<String>()
    }
}
