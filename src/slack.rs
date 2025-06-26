//! Slack integration for Human-in-the-Loop MCP server

use crate::tools::Human;
use anyhow;

/// Placeholder Slack implementation
pub struct HumanInSlack {
    _user_id: String,
    _channel_id: String,
}

impl HumanInSlack {
    pub fn new(user_id: String, channel_id: String) -> Self {
        Self {
            _user_id: user_id,
            _channel_id: channel_id,
        }
    }
}

#[async_trait::async_trait]
impl Human for HumanInSlack {
    async fn ask(&self, question: &str) -> anyhow::Result<String> {
        println!("Slack (placeholder): {}", question);
        Ok("Placeholder response from Slack".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_slack_placeholder() {
        let slack = HumanInSlack::new("U123".to_string(), "C123".to_string());
        let result = slack.ask("test question").await.unwrap();
        assert_eq!(result, "Placeholder response from Slack");
    }
}
