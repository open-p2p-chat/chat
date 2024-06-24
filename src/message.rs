use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub content: String,
}

impl Message {
    pub fn new(username: String, content: String) -> Self {
        Self { username, content }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(serialized: &str) -> Self {
        serde_json::from_str(serialized).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let message = Message::new("user1".to_string(), "Hello, World!".to_string());
        let serialized = message.serialize();
        let deserialized = Message::deserialize(&serialized);
        assert_eq!(message.username, deserialized.username);
        assert_eq!(message.content, deserialized.content);
    }
}
