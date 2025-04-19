use crate::db::Db;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::{Datetime, RecordId};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionData {
    pub id: Option<RecordId>,
    pub command: String,
    pub args: String,
    pub tgid: u32,
    pub pid: u32,
    pub gid: u32,
    pub uid: u32,
    pub timestamp: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionPaginationTotal {
    pub total: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionPaginationData {
    pub data: Vec<CommandExecutionData>,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandExecutionCountsData {
    pub command: String,
    pub total: u64,
}

impl Default for CommandExecutionData {
    fn default() -> Self {
        Self {
            id: None,
            command: String::new(),
            args: String::new(),
            tgid: 0,
            gid: 0,
            uid: 0,
            pid: 0,
            timestamp: Datetime::from(Utc::now()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandExecution {
    db: Arc<Db>,
}
impl CommandExecution {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db: db.clone() }
    }

    pub fn table() -> String {
        "command_execution".to_string()
    }

    pub async fn create(&self, data: CommandExecutionData) -> Result<CommandExecutionData, String> {
        let _ = self.db.connect().await?;
        if data.command.is_empty() {
            return Err("[COMMAND_EXECUTION ERROR] create: command must not be empty".to_string());
        }
        match self.db.get_client().read() {
            Ok(db_client) => {
                match db_client
                    .insert::<Vec<CommandExecutionData>>(Self::table())
                    .content(data)
                    .await
                {
                    Ok(data) => {
                        if let Some(value) = data.first() {
                            return Ok(value.to_owned());
                        }
                        return Err("[COMMAND_EXECUTION ERROR] create: value not found".to_string());
                    }
                    Err(error) => Err(format!(
                        "[COMMAND_EXECUTION ERROR] create: {}",
                        error.to_string()
                    )),
                }
            }
            Err(error) => Err(format!(
                "[COMMAND_EXECUTION ERROR] create: {}",
                error.to_string()
            )),
        }
    }

    pub async fn get_counts(&self) -> Result<Vec<CommandExecutionCountsData>, String> {
        let _ = self.db.connect().await?;

        match self.db.get_client().read() {
            Ok(db_client) => {
                match db_client.query("SELECT `command`,count(`command`) as total FROM type::table($table) GROUP BY `command`")
                    .bind(("table",Self::table()))
                    .await
                {
                    Ok(mut response) => {
                        match response.take::<Vec<CommandExecutionCountsData>>(0) {
                            Ok(data) => Ok(data),
                            Err(error) => Err(format!("[COMMAND_EXECUTION ERROR] get_counts: {}",error.to_string()))
                        }
                    }
                    Err(error) => Err(format!(
                        "[COMMAND_EXECUTION ERROR] get_counts: {}",
                        error.to_string()
                    )),
                }
            }
            Err(error) => Err(format!(
                "[COMMAND_EXECUTION ERROR] get_counts: {}",
                error.to_string()
            )),
        }
    }

    pub async fn get_executed_commands(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<CommandExecutionPaginationData, String> {
        let _ = self.db.connect().await?;

        match self.db.get_client().read() {
            Ok(db_client) => {
                match db_client
                    .query(
                        r#"
                    SELECT count() as total FROM type::table($table) GROUP BY count;
                    SELECT * FROM type::table($table) LIMIT $limit START $offset;
                "#,
                    )
                    .bind(("table", Self::table()))
                    .bind(("limit", limit))
                    .bind(("offset", offset))
                    .await
                {
                    Ok(mut response) => {
                        let total: usize = match response
                            .take::<Option<CommandExecutionPaginationTotal>>(0)
                        {
                            Ok(data) => match data {
                                Some(item) => item.total,
                                None => {
                                    return Err(
                                        "[COMMAND_EXECUTION ERROR] get_executed_commands: invalid total value".to_string()
                                    );
                                }
                            },
                            Err(error) => {
                                return Err(format!(
                                    "[COMMAND_EXECUTION ERROR] get_executed_commands {}",
                                    error.to_string()
                                ));
                            }
                        };
                        let data: Vec<CommandExecutionData> =
                            match response.take::<Vec<CommandExecutionData>>(1) {
                                Ok(data) => data,
                                Err(error) => {
                                    return Err(format!(
                                        "[COMMAND_EXECUTION ERROR] get_executed_commands: {}",
                                        error.to_string()
                                    ));
                                }
                            };
                        return Ok(CommandExecutionPaginationData {
                            data,
                            total,
                            offset,
                            limit,
                        });
                    }
                    Err(error) => Err(format!(
                        "[COMMAND_EXECUTION ERROR] get_executed_commands: {}",
                        error.to_string()
                    )),
                }
            }
            Err(error) => Err(format!(
                "[COMMAND_EXECUTION ERROR] get_executed_commands: {}",
                error.to_string()
            )),
        }
    }
}

#[cfg(test)]
mod test_command_execution {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_create() {
        let config = AppConfig::load(None);
        assert!(config.is_ok(), "{:?}", config.err());
        let app_config = config.unwrap();
        let database_server_config = app_config.database_server;
        let db = Db::new(Arc::new(database_server_config)).await;
        assert!(db.is_ok(), "{:?}", db.err());
        let db_instance: Arc<Db> = Arc::new(db.unwrap());
        let api = CommandExecution::new(db_instance);

        let commands: Vec<(String, String, u32, u32, u32, u32)> = Vec::from([
            ("ls".to_string(), "-ll".to_string(), 1, 2, 3, 4),
            ("docker".to_string(), "ps".to_string(), 5, 6, 7, 8),
            ("ps".to_string(), "-ef".to_string(), 9, 10, 11, 12),
            (
                "docker-compose".to_string(),
                "up".to_string(),
                13,
                14,
                15,
                16,
            ),
        ]);

        for _ in 0..5 {
            for i in &commands {
                let item = i.clone();
                let data = api
                    .create(CommandExecutionData {
                        id: None,
                        command: item.0,
                        args: item.1,
                        tgid: item.2,
                        pid: item.3,
                        gid: item.4,
                        uid: item.5,
                        ..Default::default()
                    })
                    .await;
                assert!(data.is_ok(), "{:?}", data.err());
            }
        }

        let counts = api.get_counts().await;
        assert!(counts.is_ok(), "{:?}", counts.err());
        let counts = counts.unwrap();
        assert!(counts.len() > 0, "expected counts to have atleast 1 record");

        let commands = api.get_executed_commands(10, 0).await;
        assert!(commands.is_ok(), "{:?}", commands.err());
        let data = commands.unwrap();
        assert!(
            data.data.len() > 0,
            "expected commands to have atleast 1 record"
        );
        assert!(data.total > 0, "expected commands to have atleast 1 total");
        assert!(
            data.offset == 0,
            "expected commands to have atleast 1 offset"
        );
        assert!(data.limit > 0, "expected commands to have atleast 1 limit");
    }
}
