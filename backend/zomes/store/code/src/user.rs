use hdk::prelude::*;
use crate::types::User;

pub(crate) fn register(user: User) -> ZomeApiResult<Address> {
    let entry = user.clone().entry();
    let address = hdk::commit_entry(&entry)?;
    hdk::link_entries(*hdk::AGENT_ADDRESS, &address, "user", "")?;
    Ok(address)
}