use chrono::{DateTime, Utc};
use hdk::prelude::*;
use validator::Validate;

/*
{
  "article": {
    "slug": "how-to-train-your-dragon",
    "title": "How to train your dragon",
    "description": "Ever wonder how?",
    "body": "It takes a Jacobian",
    "tagList": ["dragons", "training"],
    "createdAt": "2016-02-18T03:22:56.637Z",
    "updatedAt": "2016-02-18T03:48:35.824Z",
    "favorited": false,
    "favoritesCount": 0,
    "author": {
      "username": "jake",
      "bio": "I work at statefarm",
      "image": "https://i.stack.imgur.com/xHWG8.jpg",
      "following": false
    }
  }
}
*/


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, validator_derive::Validate)]
pub struct Author {
    pub username: String,
    pub email: String,
    pub bio: String,
    #[validate(url)]
    pub image: String,
    pub following: bool,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: usize,
    pub author: Address,
}

impl Article {
    pub(crate) fn entry_def() -> ValidatingEntryType {
        entry!(
            name: "article",
            description: "A single article",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Article>| {
                Ok(())
            },
            links: [
                from!(
                    "%agent_id",
                    link_type: "articles",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    pub(crate) fn entry(self) -> Entry {
        Entry::App("article".into(), self.into())
    }
}

impl Author {
    pub(crate) fn entry_def() -> ValidatingEntryType {
        entry!(
            name: "author",
            description: "A article author",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Author>| {
                Ok(())
            },
            links: [
                from!(
                    "%agent_id",
                    link_type: "author",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    pub(crate) fn entry(self) -> Entry {
        Entry::App("author".into(), self.into())
    }
}
