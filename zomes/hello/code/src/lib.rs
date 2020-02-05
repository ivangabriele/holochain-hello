#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
};

use hdk_proc_macros::zome;

#[zome]
mod hello_zome {

    #[init]
    fn init() {
        Ok(())
    }

    /** Check if the user is allowed to join the network. */
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    pub fn hello() -> ZomeApiResult<String> {
      Ok("Hello Holochain!".into())
    }

}
