use crate::db::Db;
use crate::enums::ip_protocol::IpProtocol;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::RecordId;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallRuleData {
    pub id: Option<RecordId>,
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub cidr: u16,
    pub layer: u8,
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
}
impl Default for FirewallRuleData {
    fn default() -> Self {
        Self {
            id: None,
            ip: [0; 4],
            cidr: 0,
            layer: 4,
            protocol: IpProtocol::Undefined,
            from_port: None,
            to_port: None,
            status: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    db: Arc<Db>,
}

impl FirewallRule {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db: db.clone() }
    }

    pub fn table() -> String {
        "firewall_rule".to_string()
    }

    pub async fn create(&self, data: FirewallRuleData) -> Result<FirewallRuleData, String> {
        let _ = self.db.connect().await?;
        match self.db.get_client().read() {
            Ok(client) => {
                match client
                    .insert::<Vec<FirewallRuleData>>(Self::table())
                    .content(data)
                    .await
                {
                    Ok(data) => {
                        if let Some(value) = data.first() {
                            return Ok(value.to_owned());
                        }
                        return Err("[FIREWALL_RULE ERROR] create: value not found".to_string());
                    }
                    Err(error) => {
                        return Err(format!(
                            "[FIREWALL_RULE ERROR] create {}",
                            error.to_string()
                        ))
                    }
                }
            }
            Err(error) => Err(format!(
                "[FIREWALL_RULE ERROR] create {}",
                error.to_string()
            )),
        }
    }

    pub async fn remove(&self, id: RecordId) -> Result<FirewallRuleData, String> {
        let _ = self.db.connect().await?;
        match self.db.get_client().read() {
            Ok(client) => match client.delete::<Option<FirewallRuleData>>(id).await {
                Ok(data) => {
                    if let Some(value) = data {
                        return Ok(value.to_owned());
                    }
                    return Err("[FIREWALL_RULE ERROR] remove: data not found".to_string());
                }

                Err(error) => {
                    return Err(format!(
                        "[FIREWALL_RULE ERROR] remove {}",
                        error.to_string()
                    ))
                }
            },
            Err(error) => Err(format!(
                "[FIREWALL_RULE ERROR] remove {}",
                error.to_string()
            )),
        }
    }

    pub async fn list(&self, layer: u8) -> Result<Vec<FirewallRuleData>, String> {
        let _ = self.db.connect().await?;

        match self.db.get_client().read() {
            Ok(db_client) => {
                match db_client
                    .query("SELECT * FROM type::table($table) WHERE layer=$layer;")
                    .bind(("table", Self::table()))
                    .bind(("layer", layer))
                    .await
                {
                    Ok(mut response) => match response.take::<Vec<FirewallRuleData>>(0) {
                        Ok(data) => Ok(data),
                        Err(error) => Err(format!(
                            "[FIREWALL_RULE ERROR]  list: {}",
                            error.to_string()
                        )),
                    },
                    Err(error) => Err(format!("[FIREWALL_RULE ERROR] list: {}", error.to_string())),
                }
            }
            Err(error) => Err(format!(
                "[FIREWALL_RULE ERROR] get_counts: {}",
                error.to_string()
            )),
        }
    }
}

#[cfg(test)]
mod test_firewall_rule {
    use super::*;
    use crate::config::AppConfig;
    #[tokio::test]
    async fn test_crud() {
        let config = AppConfig::load(None);
        assert!(config.is_ok(), "{:?}", config.err());
        let config = config.unwrap();
        let database_server_config = config.database_server;
        let db = Db::new(Arc::new(database_server_config)).await;
        assert!(db.is_ok(), "{:?}", db.err());
        let db = Arc::new(db.unwrap());
        let api = FirewallRule::new(db.clone());
        let data: FirewallRuleData = FirewallRuleData {
            id: None,
            ip: [192, 168, 211, 128],
            cidr: 32,
            layer: 4,
            status: false,
            protocol: IpProtocol::Tcp,
            from_port: Some(2000),
            to_port: Some(3000),
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());
        let created = result.unwrap();
        let result = api.list(4).await;
        assert!(result.is_ok(), "{:?}", result.err());
        let data = result.unwrap();
        assert!(data.len() > 0, "expected atleast 1 record");
        let removed = api.remove(created.id.unwrap()).await;
        assert!(removed.is_ok(), "{:?}", removed);

        let data: FirewallRuleData = FirewallRuleData {
            id: None,
            ip: [192, 168, 211, 1],
            cidr: 32,
            layer: 4,
            status: false,
            protocol: IpProtocol::Tcp,
            from_port: Some(2000),
            to_port: Some(3000),
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());

        let data: FirewallRuleData = FirewallRuleData {
            id: None,
            ip: [192, 168, 211, 1],
            cidr: 32,
            layer: 3,
            status: false,
            protocol: IpProtocol::Icmp,
            from_port: None,
            to_port: None,
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
