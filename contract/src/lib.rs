use near_sdk::{near_bindgen, PanicOnDefault, serde_json};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

// **************************************
#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Banner {
    w: usize,
    h: usize,
    pos: usize,
    battr: Vec<usize>,
    api: Vec<usize>,
    topframe: usize,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Imp {
    id: String,
    tagid: String,
    iframebuster: Vec<String>,
    banner: Banner,
}
// **************************************

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Publisher {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ext {
    storerating: usize,
    appstoreid: String,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct App {
    id: String,
    cat: Vec<String>,
    name: String,
    domain: String,
    privacypolicy: usize,
    publisher: Publisher,
    ext: Ext,
}

// **************************************
#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtDevice {
    latlonconsent: usize,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Geo {
    country: String,
    region: String,
    tp: usize,
    ext: ExtDevice,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Device {
    make: String,
    model: String,
    os: String,
    osv: String,
    ua: String,
    ip: String,
    language: String,
    devicetype: usize,
    js: usize,
    connectiontype: usize,
    dpidsha1: String,
    carrier: String,
    geo: Geo,
}

// **************************************
#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtUser {
    sessiondepth: usize,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    id: String,
    ext: ExtUser,
}

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Channel {
    id: String,
    at: usize,
    tmax: usize,
    imp: Vec<Imp>,
    app: App,
    device: Device,
    user: User,
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

    pub fn add_item_rubicon(&mut self,
                            channel: String,
    ) {
        let channel: Channel = serde_json::from_slice(channel.as_ref()).unwrap();
        self.adv_channel.insert(&channel.id.clone(), &channel);
    }

    pub fn get_item(&self, item_id: String) -> String {
        match self.adv_channel.get(&item_id) {
            Some(val) => {
                serde_json::to_string(&val).unwrap()
            }
            None => "not found".to_string()
        }
    }

    pub fn all_keys(&self) -> String {
        let all_keys = self.adv_channel.keys_as_vector();
        let mut res_string: String = "".into();
        for k in all_keys.iter() {
            let res_key = format!("{};", k);
            res_string.push_str(&res_key);
        }
        res_string
    }

    pub fn get_item_oracle(&self) -> String {
        "item oracle success".to_string()
    }
}