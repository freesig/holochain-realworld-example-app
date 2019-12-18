use crate::types::Author;
use hdk::prelude::*;

pub fn register(author: Author) -> ZomeApiResult<Address> {
    let entry = author.clone().entry();
    let address = hdk::commit_entry(&entry)?;
    hdk::link_entries(*hdk::AGENT_ADDRESS, &address, "author", "")?;
    Ok(address)
}
