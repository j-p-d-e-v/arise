use crate::db::Db;
use network_types::ip::IpProto;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::RecordId;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallRuleData {
    pub ip: [u8; 4],
    pub port: u16,
    pub protocol: IpProto,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    db: Arc<Db>,
}

impl FirewallRule {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create(data: FirewallRuleData ) -> Result<FirewallRuleData, String> {
        todo!();
    }

    pub fn async remove(id: RecordId) -> Result<bool,String> {
        todo!();
    }

    pub fn async list() -> Result<Vec<FirewallRuleData>, String> {
        todo!();
    }
}
