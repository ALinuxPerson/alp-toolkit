use std::error::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cfg(feature = "toml")]
mod toml {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Toml {}

    impl Format for Toml {
        type SerializeError = toml::ser::Error;
        type DeserializeError = toml::de::Error;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            toml::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            toml::to_string(t)
        }
    }
}

#[cfg(feature = "toml")]
pub use toml::Toml;

#[cfg(feature = "json")]
mod json {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Json {}

    impl Format for Json {
        type SerializeError = serde_json::Error;
        type DeserializeError = serde_json::Error;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            serde_json::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            serde_json::to_string(t)
        }
    }
}

#[cfg(feature = "json")]
pub use json::Json;

#[cfg(feature = "yaml")]
mod yaml {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Yaml {}

    impl Format for Yaml {
        type SerializeError = serde_yaml::Error;
        type DeserializeError = serde_yaml::Error;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            serde_yaml::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            serde_yaml::to_string(t)
        }
    }
}

#[cfg(feature = "yaml")]
pub use yaml::Yaml;

#[cfg(feature = "ini")]
mod ini {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Ini {}

    impl Format for Ini {
        type SerializeError = serde_ini::ser::Error;
        type DeserializeError = serde_ini::de::Error;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            serde_ini::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            serde_ini::to_string(&t)
        }
    }
}

#[cfg(feature = "ini")]
pub use ini::Ini;

#[cfg(feature = "ron")]
mod ron {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Ron {}

    impl Format for Ron {
        type SerializeError = ron::Error;
        type DeserializeError = ron::de::SpannedError;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            ron::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            ron::to_string(t)
        }
    }
}

#[cfg(feature = "ron")]
pub use ron::Ron;

#[cfg(feature = "json5")]
mod json5 {
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use crate::formats::Format;

    pub enum Json5 {}

    impl Format for Json5 {
        type SerializeError = json5::Error;
        type DeserializeError = json5::Error;

        fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError> {
            json5::from_str(s)
        }

        fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError> {
            json5::to_string(t)
        }
    }
}

#[cfg(feature = "json5")]
pub use json5::Json5;

pub trait Format {
    type SerializeError: Error;
    type DeserializeError: Error;

    fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::DeserializeError>;
    fn to_string<T: Serialize>(t: &T) -> Result<String, Self::SerializeError>;
}
