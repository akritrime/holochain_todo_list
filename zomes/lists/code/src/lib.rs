#[macro_use]
extern crate hdk;
// #[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hdk::holochain_dna::zome::entry_types::Sharing;
use hdk::holochain_core_types::hash::HashString;
use hdk::holochain_core_types::entry::Entry;
use hdk::holochain_core_types::entry_type::EntryType;
use hdk::holochain_core_types::cas::content::Address;
// use serde::Serialize;
use hdk::holochain_core_types::json::JsonString;


// see https://holochain.github.io/rust-api/0.0.1/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug)]
struct List {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ListItem {
    text: String,
    completed: bool
}

fn handle_create_list(list: List) -> JsonString {
    let entry = Entry::new(EntryType::App("list".into()), json!(list));
    match hdk::commit_entry(&entry) {
        Ok(address) => JsonString::from(json!({
            "success": true,
            "address": address
        })),

        Err(hdk_err) => JsonString::from(json!({
            "success": false,
            "error": hdk_err
        }))
    }
}

fn handle_add_item(list_item: ListItem, list_addr: HashString) -> JsonString {
    let entry = Entry::new(EntryType::App("listItem".into()), json!(list_item));
    match hdk::commit_entry(&entry)
        .and_then(|item_addr| {
            hdk::link_entries(&list_addr, &item_addr, "items")
        })
    {
        Ok(_) => {
            JsonString::from(json!({"success": true}))
        },
        Err(hdk_err) => JsonString::from(json!({"success": false, "error": hdk_err}))
    }
}

fn handle_get_list(list_addr: Address) -> JsonString {
    match hdk::get_entry(list_addr.clone()) {
        Ok(Some(_list)) => {
            match hdk::get_links(&list_addr, "items") {
                Ok(result) => {
                    JsonString::from(json!({
                        "success": true,
                        "result": result
                    }))
                }

                Err(hdk_err) => JsonString::from(json!({
                    "success": false,
                    "error": hdk_err
                }))
            }
        },

        Ok(None) => JsonString::from(json!({"success": false, "error": "no list found"})),
        Err(hdk_err) => JsonString::from(json!({
            "success": false,
            "error": hdk_err
        }))
    }
}

define_zome! {
    entries: [
        entry!(
            name: "list",
            description: "",
            sharing: Sharing::Public,
            native_type: List,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list: List, _ctx: hdk::ValidationData| {
                Ok(())
            }
        ),
        entry!(
            name: "listItem",
            description: "",
            sharing: Sharing::Public,
            native_type: ListItem,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |list_item: ListItem, _ctx:  hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_list: {
                inputs: |list: List|,
                outputs: |result: JsonString|,
                handler: handle_create_list
            }
            add_item: {
                inputs: |list_item: ListItem, list_addr: HashString|,
                outputs: |result: JsonString|,
                handler: handle_add_item
            }
            get_list: {
                inputs: |list_addr: HashString|,
                outputs: |result: JsonString|,
                handler: handle_get_list
            }
        }
    }
}
