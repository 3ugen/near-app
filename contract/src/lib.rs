use near_sdk::{near_bindgen, PanicOnDefault, serde_json};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::Serialize;

#[derive(Debug, Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Channel {
    item_id: String,
    model: String,
    direct: usize,
    email: usize,
    fb: usize,
    g_search: usize,
    organic: usize,
    youtube: usize,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    adv_channel: UnorderedMap<String, Channel>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {
            adv_channel: UnorderedMap::new(b'a')
        }
    }

    pub fn add_item(&mut self,
                    item_id: String,
                    direct: usize,
                    email: usize,
                    fb: usize,
                    g_search: usize,
                    organic: usize,
                    youtube: usize,
    ) {
        self.adv_channel.insert(&item_id.clone(), &Channel {
            item_id,
            model: "LastInteraction".to_string(),
            direct,
            email,
            fb,
            g_search,
            organic,
            youtube,
        });
    }

    pub fn get_item(&self, item_id: String) -> String {
        match self.adv_channel.get(&item_id) {
            Some(val) => {
                serde_json::to_string(&val).unwrap()
            }
            None => "not found".to_string()
        }
    }
}