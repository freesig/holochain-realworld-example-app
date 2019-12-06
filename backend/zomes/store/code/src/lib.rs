#![feature(proc_macro_hygiene)]

use chrono::{DateTime, FixedOffset};
use hdk::prelude::*;
use hdk_proc_macros::zome;

use types::{Article, Articles, Author};


#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "article",
            description: "A single article",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Article>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn list_articles() -> ZomeApiResult<Articles> {
        let author = Author {
            username: "jake".into(),
            bio: "I work at statefarm".into(),
            image: "https://i.stack.imgur.com/xHWG8.jpg".into(),
            following: false,
        };
        let created_at: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2016-02-18T03:22:56.637Z")
            .map_err(|_| ZomeApiError::from(format!("Bad timezone")))?;
        let updated_at = DateTime::parse_from_rfc3339("2016-02-18T03:48:35.824Z")
            .map_err(|_| ZomeApiError::from(format!("Bad timezone")))?;
        let article = Article {
            slug: "how-to-train-your-dragon".into(),
            title: "How to train your dragon".into(),
            description: "Ever wonder how?".into(),
            body: "It takes a Jacobian".into(),
            tag_list: vec!["dragons".into(), "training".into()],
            created_at: created_at,
            updated_at: updated_at,
            favorited: false,
            favorites_count: 0,
            author: author,
        };
        Ok(Articles {
            articles: vec![article],
            article_count: 1,
        })
    }

}
