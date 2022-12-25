pub enum Command {
    Rename(String),
    Unknown
}

impl Command {
    pub fn from(text: &str) -> Option<Self> {
        if !text.starts_with("/") {
            return None;
        }

        if text.starts_with("/name") {
            return Some(Self::Rename(text.replace("/name", "").trim().to_string()));
        }

        Some(Self::Unknown)
    }
}