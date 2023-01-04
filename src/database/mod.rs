mod mongodb;

use ::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

///This should be a collection(document)
#[async_trait]
pub trait Database<T: Serialize + DeserializeOwned + Send + Sync + Unpin> {
    /// Insert a structure value into database
    /// Return: Ok(()); Err(Error string)
    async fn abstract_insert(&self, insert_doc: T) -> Result<(), String>;
    /// Delete a structure value from database
    /// Return: Ok(Delete count); Err(Error string)
    async fn abstract_delete(&self, filter_doc: Document) -> Result<u64, String>;
    /// Queue a structure from database
    /// Values which can be none should be wrapped in Option
    /// Return: None if not found any; Some(Results in vector)
    async fn abstract_find(&self, find_doc: Document) -> Option<Vec<T>>;
    /// Update a structure in database
    /// Values which can be none should be wrapped in Option
    /// Return: Ok(Modify count); Err(Error string)
    async fn abstract_update(&self, filter_doc: Document, update_doc: Document) -> Result<u64, String>;
}
