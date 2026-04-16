use futures::stream::TryStreamExt;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::errors::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub gender: String,
    pub gender_probability: f64,
    pub sample_size: u64,
    pub age: u8,
    pub age_group: String,
    pub country_id: String,
    pub country_probability: f64,
    pub created_at: String,
}

#[derive(Debug, Default)]
pub struct ProfileFilters {
    pub gender: Option<String>,
    pub country_id: Option<String>,
    pub age_group: Option<String>,
}

#[derive(Clone)]
pub struct ProfileRepo {
    collection: Collection<Profile>,
}

impl std::fmt::Debug for ProfileRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProfileRepo").finish()
    }
}

impl ProfileRepo {
    pub fn new(db: &mongodb::Database) -> Self {
        Self {
            collection: db.collection("profiles"),
        }
    }

    pub async fn find_by_name(&self, name: &str) -> Result<Option<Profile>> {
        self.collection
            .find_one(mongodb::bson::doc! { "name": name })
            .await
            .map_err(|e| {
                crate::errors::AppError::ServiceUnavailable(format!("DB Search Error: {}", e))
            })
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Profile>> {
        self.collection
            .find_one(mongodb::bson::doc! { "id": id })
            .await
            .map_err(|e| {
                crate::errors::AppError::ServiceUnavailable(format!("DB Search Error: {}", e))
            })
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool> {
        let result = self
            .collection
            .delete_one(mongodb::bson::doc! { "id": id })
            .await
            .map_err(|e| {
                crate::errors::AppError::ServiceUnavailable(format!("DB Delete Error: {}", e))
            })?;
        Ok(result.deleted_count > 0)
    }

    pub async fn find_all(&self, filters: ProfileFilters) -> Result<Vec<Profile>> {
        let mut filter_doc = mongodb::bson::doc! {};

        if let Some(gender) = filters.gender {
            filter_doc.insert(
                "gender",
                mongodb::bson::doc! { "$regex": format!("^{}$", gender), "$options": "i" },
            );
        }

        if let Some(country) = filters.country_id {
            filter_doc.insert(
                "country_id",
                mongodb::bson::doc! { "$regex": format!("^{}$", country), "$options": "i" },
            );
        }

        if let Some(age) = filters.age_group {
            filter_doc.insert(
                "age_group",
                mongodb::bson::doc! { "$regex": format!("^{}$", age), "$options": "i" },
            );
        }

        let cursor = self.collection.find(filter_doc).await.map_err(|e| {
            crate::errors::AppError::ServiceUnavailable(format!("DB List Error: {}", e))
        })?;

        cursor.try_collect().await.map_err(|e| {
            crate::errors::AppError::ServiceUnavailable(format!("DB Cursor Error: {}", e))
        })
    }

    pub async fn insert_profile(&self, profile: Profile) -> Result<()> {
        self.collection.insert_one(profile).await.map_err(|e| {
            crate::errors::AppError::ServiceUnavailable(format!("DB Insert Error: {}", e))
        })?;
        Ok(())
    }
}
