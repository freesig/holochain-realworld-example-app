#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

mod articles;
mod profile;
mod user;
mod types;

use types::{Article, Profile, User};

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
    fn profile_def() -> ValidatingEntryType {
        Profile::entry_def()
    }
    
    #[entry_def]
    fn user_def() -> ValidatingEntryType {
        User::entry_def()
    }

    #[entry_def]
    fn article_def() -> ValidatingEntryType {
        Article::entry_def()
    }

    #[zome_fn("hc_public")]
    pub fn list_articles() -> ZomeApiResult<Vec<Article>> {
        hdk::utils::get_links_and_load_type(
            *hdk::AGENT_ADDRESS,
            LinkMatch::Exactly("articles"),
            LinkMatch::Any,
        )
    }

    #[zome_fn("hc_public")]
    pub fn create_article(article: Article) -> ZomeApiResult<Address> {
        articles::create_article(article)
    }

    #[zome_fn("hc_public")]
    pub fn get_profile(address: Address) -> ZomeApiResult<Profile> {
        hdk::utils::get_as_type(address)
    }
    
    #[zome_fn("hc_public")]
    pub fn get_user(address: Address) -> ZomeApiResult<User> {
        hdk::utils::get_as_type(address)
    }

    #[zome_fn("hc_public")]
    pub fn register(author: Profile) -> ZomeApiResult<Address> {
        profile::register(author)
    }
    
    #[zome_fn("hc_public")]
    pub fn get_article(address: Address) -> ZomeApiResult<Article> {
        hdk::utils::get_as_type(address)
    }

    #[zome_fn("hc_public")]
    pub fn get_me() -> ZomeApiResult<Address> {
        profile::get_me()
    }

}
