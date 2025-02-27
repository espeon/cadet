use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JetstreamEndpointLocations {
    UsEast,
    UsWest,
}

impl ToString for JetstreamEndpointLocations {
    fn to_string(&self) -> String {
        match self {
            Self::UsEast => "us-east".into(),
            Self::UsWest => "us-west".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JetstreamEndpoints {
    Public(JetstreamEndpointLocations, i8),
    Custom(String),
}

impl Display for JetstreamEndpoints {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Public(location, id) => write!(
                f,
                "wss://jetstream{}.{}.bsky.network/subscribe",
                id,
                location.to_string()
            ),
            Self::Custom(url) => write!(f, "{}", url),
        }
    }
}

impl Default for JetstreamEndpoints {
    fn default() -> Self {
        Self::Public(JetstreamEndpointLocations::UsEast, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_public() {
        let endpoint = JetstreamEndpoints::Public(JetstreamEndpointLocations::UsEast, 2);
        assert_eq!(
            endpoint.to_string(),
            "wss://jetstream2.us-east.bsky.network/subscribe"
        );
    }

    #[test]
    fn test_display_custom() {
        let endpoint = JetstreamEndpoints::Custom("wss://custom.bsky.network/subscribe".into());
        assert_eq!(endpoint.to_string(), "wss://custom.bsky.network/subscribe");
    }
}
