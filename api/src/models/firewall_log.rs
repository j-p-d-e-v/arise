use crate::db::Db;
use crate::enums::ip_protocol::IpProtocol;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{Datetime, RecordId};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallLogData {
    pub id: Option<RecordId>,
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub port: Option<u16>,
    pub status: bool,
    pub timestamp: Datetime,
}
impl Default for FirewallLogData {
    fn default() -> Self {
        Self {
            id: None,
            ip: [0; 4],
            port: None,
            protocol: IpProtocol::Undefined,
            status: false,
            timestamp: Datetime::from(Utc::now()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirewallLog {
    db: Arc<Db>,
}

impl FirewallLog {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db: db.clone() }
    }

    pub fn table() -> String {
        "firewall_log".to_string()
    }

    pub async fn create(&self, data: FirewallLogData) -> Result<FirewallLogData, String> {
        let _ = self.db.connect().await?;
        match self.db.get_client().read() {
            Ok(client) => {
                match client
                    .insert::<Vec<FirewallLogData>>(Self::table())
                    .content(data)
                    .await
                {
                    Ok(data) => {
                        if let Some(value) = data.first() {
                            return Ok(value.to_owned());
                        }
                        return Err("[FIREWALL_LOG ERROR] create: value not found".to_string());
                    }
                    Err(error) => {
                        return Err(format!("[FIREWALL_LOG ERROR] create {}", error.to_string()))
                    }
                }
            }
            Err(error) => Err(format!("[FIREWALL_LOG ERROR] create {}", error.to_string())),
        }
    }

    pub async fn list(&self, status: bool, limit: usize) -> Result<Vec<FirewallLogData>, String> {
        let _ = self.db.connect().await?;

        match self.db.get_client().read() {
            Ok(db_client) => {
                match db_client
                    .query("SELECT * FROM type::table($table) WHERE status=$status LIMIT $limit;")
                    .bind(("table", Self::table()))
                    .bind(("status", status))
                    .bind(("limit", limit))
                    .await
                {
                    Ok(mut response) => match response.take::<Vec<FirewallLogData>>(0) {
                        Ok(data) => Ok(data),
                        Err(error) => {
                            Err(format!("[FIREWALL_LOG ERROR]  list: {}", error.to_string()))
                        }
                    },
                    Err(error) => Err(format!("[FIREWALL_LOG ERROR] list: {}", error.to_string())),
                }
            }
            Err(error) => Err(format!(
                "[FIREWALL_LOG ERROR] get_counts: {}",
                error.to_string()
            )),
        }
    }
}

#[cfg(test)]
mod test_firewall_log {

    use super::*;
    use crate::config::AppConfig;
    #[tokio::test]
    async fn test_logging() {
        let config = AppConfig::load(None);
        assert!(config.is_ok(), "{:?}", config.err());
        let config = config.unwrap();
        let database_server_config = config.database_server;
        let db = Db::new(Arc::new(database_server_config)).await;
        assert!(db.is_ok(), "{:?}", db.err());
        let db = Arc::new(db.unwrap());
        let api = FirewallLog::new(db.clone());
        let data: FirewallLogData = FirewallLogData {
            id: None,
            ip: [192, 168, 211, 128],
            status: false,
            protocol: IpProtocol::Tcp,
            port: Some(3000),
            timestamp: Datetime::from(Utc::now()),
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());
        let result = api.list(false, 9999).await;
        assert!(result.is_ok(), "{:?}", result.err());
        let data = result.unwrap();
        assert!(data.len() > 0, "expected atleast 1 record");
        let data: FirewallLogData = FirewallLogData {
            id: None,
            ip: [192, 168, 211, 1],
            status: false,
            protocol: IpProtocol::Tcp,
            port: Some(2000),
            timestamp: Datetime::from(Utc::now()),
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());

        let data: FirewallLogData = FirewallLogData {
            id: None,
            ip: [192, 168, 211, 1],
            status: false,
            protocol: IpProtocol::Icmp,
            port: None,
            timestamp: Datetime::from(Utc::now()),
        };
        let result = api.create(data).await;
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
