use anyhow::{anyhow, Error};
use aya::maps::{HashMap, MapData};
use log::warn;

use crate::{api::Api, rule::FirewallRuleData};

pub async fn configure_firewall_cidrs(
    api: &Api,
    layer: u8,
    firewall_cidrs: &mut HashMap<&mut MapData, u16, u16>,
) -> Result<(), Error> {
    let data: Vec<FirewallRuleData> = match api.load_firewall_rules(layer).await {
        Ok(value) => value,
        Err(error) => return Err(anyhow!(error.to_string())),
    };
    for item in data {
        if let Err(_) = firewall_cidrs.get(&item.cidr, 0) {
            if let Err(error) = firewall_cidrs.insert(&item.cidr, item.cidr, 0) {
                warn!("[FIREWALL CIDRS] {}", error.to_string());
            }
        }
    }
    Ok(())
}
