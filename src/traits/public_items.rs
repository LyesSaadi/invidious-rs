use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fmt::Debug};

use crate::errors::*;
#[cfg(any(feature = "sync", feature = "async"))]
use crate::{functions::*, structs::*};

/// A trait which contains all basic functionalites a binding struct should include.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait PublicItems {
    // Error gate keeping
    /// Converts &str to Self.
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        let value: Value = serde_json::from_str(s)?;

        match &value["error"] {
            Value::String(s) => Err(Box::new(InvidiousError::InvalidRequest(s.clone()))),
            _ => Ok(Self::from_value(value)?),
        }
    }

    /// Blocking function to fetch the json data and deserialising it into Self.
    #[cfg(feature = "sync")]
    fn fetch_sync(
        client: &ClientSync,
        id: Option<&str>,
        params: Option<&str>,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        let url = Self::url(&client.instance, url_params(id, params));
        let res = client.method.fetch(&url)?;
        Self::from_str(&res)
    }

    /// Async function to fetch the json data and deserialising it into Self.
    #[cfg(feature = "async")]
    async fn fetch_async(
        client: &ClientAsync,
        id: Option<&str>,
        params: Option<&str>,
    ) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        let url = Self::url(&client.instance, url_params(id, params));
        let res = client.method.fetch(&url).await?;
        Self::from_str(&res)
    }

    /// Converts Value to Self.
    fn from_value(value: Value) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + DeserializeOwned,
    {
        Ok(serde_json::from_value(value)?)
    }

    /// Converts Self to a string.
    fn to_string<'a>(&self) -> String
    where
        Self: Serialize + Deserialize<'a> + Debug,
    {
        serde_json::to_string(self).unwrap()
    }

    /// Returns the endpoint url.
    fn url(server: &str, params: String) -> String;
}
