use hdk::prelude::*;

use crate::types::Article;

pub(crate) fn create_article(article: Article) -> ZomeApiResult<Address> {
    let entry = article.entry();
    let address = hdk::commit_entry(&entry)?;
    hdk::link_entries(*hdk::AGENT_ADDRESS, &address, "articles", "")?;
    Ok(address)
}
