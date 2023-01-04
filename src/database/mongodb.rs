use mongodb::bson::{doc, Document};
use mongodb::Collection;
use serde::{Serialize};
use crate::database::Database;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use serde::de::DeserializeOwned;

#[async_trait]
impl<T: Serialize + DeserializeOwned + Send + Sync + Unpin> Database<T> for Collection<T> {
    async fn abstract_insert(&self, insert_doc: T) -> Result<(), String> {
        match self.insert_one(insert_doc, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn abstract_delete(&self, filter_doc: Document) -> Result<u64, String> {
        match self.delete_many(filter_doc, None).await {
            Ok(a) => Ok(a.deleted_count),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn abstract_find(&self, find_doc: Document) -> Option<Vec<T>> {
        match self.find(find_doc, None).await {
            Ok(mut a) => {
                let mut res: Vec<T> = vec![];
                while let Some(doc) = a.try_next().await.unwrap() {
                    res.push(doc);
                }
                Some(res)
            }
            Err(e) => None,
        }
    }

    async fn abstract_update(&self, filter_doc: Document, update_doc: Document) -> Result<u64, String> {
        match self.update_many(filter_doc, doc! {"$set": update_doc}, None).await {
            Ok(a) => Ok(a.modified_count),
            Err(e) => Err(e.to_string()),
        }
    }
}