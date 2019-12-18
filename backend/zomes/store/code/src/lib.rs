#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

mod articles;
mod types;

use types::{Article, Author};

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
    fn author_def() -> ValidatingEntryType {
        Author::entry_def()
    }
    
    #[entry_def]
    fn article_def() -> ValidatingEntryType {
        Article::entry_def()
    }

    #[zome_fn("hc_public")]
    pub fn list_articles() -> ZomeApiResult<Vec<Article>> {
        hdk::utils::get_links_and_load_type(*hdk::AGENT_ADDRESS, LinkMatch::Exactly("articles"), LinkMatch::Any)
    }

    #[zome_fn("hc_public")]
    pub fn create_article(article: Article) -> ZomeApiResult<Address> {
        articles::create_article(article)
    }
    
    #[zome_fn("hc_public")]
    pub fn get_author(address: Address) -> ZomeApiResult<Author> {
        hdk::utils::get_as_type(address)
    }
    
    #[zome_fn("hc_public")]
    pub fn get_me() -> ZomeApiResult<Author> {
        let author = hdk::utils::get_links_and_load_type(*hdk::AGENT_ADDRESS, LinkMatch::Exactly("author"), LinkMatch::Any)?;
        author.into_iter().next().ok_or_else(|| ZomeApiError::Internal("Missing Author".into()))
    }
    
}
