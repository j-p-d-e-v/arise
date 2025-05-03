use anyhow::{anyhow, Error};
use aya::maps::{
    lpm_trie::{Key, LpmTrie},
    MapData,
};
use log::{info, warn};

use crate::{
    api::Api,
    protocol::get_protocol,
    rule::{FirewallRuleData, Rule},
};

pub async fn configure_firewall_rules(
    api: &Api,
    layer: u8,
    firewall_rules: &mut LpmTrie<&mut MapData, [u8; 4], Rule>,
) -> Result<(), Error> {
    let data: Vec<FirewallRuleData> = match api.load_firewall_rules(layer).await {
        Ok(value) => value,
        Err(error) => return Err(anyhow!(error.to_string())),
    };

    let mut remove_keys: Vec<Key<[u8; 4]>> = Vec::new();
    for i in firewall_rules.iter() {
        if let Ok(item) = i {
            remove_keys.push(item.0);
        }
    }

    info!("[FIREWALL RULES] Updating Firewall Rules");
    if remove_keys.len() > 0 {
        for key in remove_keys {
            if let Err(error) = firewall_rules.remove(&key) {
                warn!("[FIREWALL RULES WARN] {:?}", error);
            }
        }
    }
    for item in data {
        let key = Key::new(item.cidr as u32, item.ip);
        let rule: Rule = Rule {
            from_port: item.from_port,
            to_port: item.to_port,
            status: item.status,
            protocol: get_protocol(item.protocol),
        };
        if let Err(error) = firewall_rules.insert(&key, &rule, 0) {
            warn!("[FIREWALL RULES WARN] {:?}", error);
        }
    }
    Ok(())
}
