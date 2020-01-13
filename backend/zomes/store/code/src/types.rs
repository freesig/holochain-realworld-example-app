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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, validator_derive::Validate)]
pub struct User {
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub profile: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, Default, validator_derive::Validate)]
pub struct Profile {
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
            validation: | validation_data: hdk::EntryValidationData<Article>| {
                match validation_data {
                    EntryValidationData::Create{ entry, ..} => entry.validate(),
                    EntryValidationData::Modify{ new_entry, ..} => new_entry.validate(),
                    _ => Ok(()),
                }
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

    fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            Err("Title cannot be empty".into())
        } else if self.description.is_empty() {
            Err("Description cannot be empty".into())
        } else if self.body.is_empty() {
            Err("Body cannot be empty".into())
        } else {
            Ok(())
        }
    }
}

impl Profile {
    pub(crate) fn entry_def() -> ValidatingEntryType {
        entry!(
            name: "profile",
            description: "A users profile",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Profile>| {
                Ok(())
            }
        )
    }

    pub(crate) fn entry(self) -> Entry {
        Entry::App("profile".into(), self.into())
    }
}

impl User {
    pub (crate) fn new(user_request: UserRequest, profile: Address) -> Self {
        User{
            username: user_request.username,
            email: user_request.email,
            profile,
        }
    }

    pub(crate) fn entry_def() -> ValidatingEntryType {
        entry!(
            name: "user",
            description: "A user",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<User>| {
                match validation_data {
                    EntryValidationData::Create{ entry, ..} => entry.validate().map_err(|e| format!("{:?}", e)),
                    EntryValidationData::Modify{ new_entry, ..} => new_entry.validate().map_err(|e| format!("{:?}", e)),
                    _ => Ok(())
                }
            },
            links: [
                from!(
                    "%agent_id",
                    link_type: "user",
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
        Entry::App("user".into(), self.into())
    }
}