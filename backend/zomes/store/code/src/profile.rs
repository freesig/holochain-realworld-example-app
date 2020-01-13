use crate::types::{Profile, UserRequest, User};
use crate::user;
use hdk::prelude::*;

pub(crate) fn register(profile: Profile) -> ZomeApiResult<Address> {
    let entry = profile.entry();
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub(crate) fn get_me() -> ZomeApiResult<Address> {
    let user = hdk::get_links(
        *hdk::AGENT_ADDRESS,
        LinkMatch::Exactly("user"),
        LinkMatch::Any,
    )?;
    match user.addresses().into_iter().next() {
        Some(user_address) => Ok(user_address),
        None => {
            let user_request = mock_persona();
            let profile_address = register(Profile::default())?;
            let user = User::new(user_request, profile_address.clone());
            user::register(user)
        }
    }
}

fn mock_persona() -> UserRequest {
    UserRequest{
        username: "bob".into(),
        email: "bob@bob2.bob".into()
    }
}