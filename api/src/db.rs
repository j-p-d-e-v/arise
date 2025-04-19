use crate::config::DatabaServerConfig;
use std::sync::{Arc, RwLock};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Debug, Clone)]
pub struct Db {
    config: Arc<DatabaServerConfig>,
    instance: Arc<RwLock<Surreal<Client>>>,
}

impl Db {
    pub fn get_client(&self) -> Arc<RwLock<Surreal<Client>>> {
        self.instance.clone()
    }

    pub async fn test_query(&self) -> Result<bool, String> {
        match self.get_client().read() {
            Ok(instance) => {
                match instance
                    .query("DEFINE TABLE IF NOT EXISTS test_table")
                    .await
                {
                    Ok(_) => Ok(true),
                    Err(error) => Err(format!("[DB_ERROR] test_query: {}", error.to_string())),
                }
            }
            Err(error) => Err(format!("db error: {}", error.to_string())),
        }
    }

    pub async fn new(config: Arc<DatabaServerConfig>) -> Result<Self, String> {
        Ok(Self {
            config: config.clone(),
            instance: Arc::new(RwLock::new(Surreal::init())),
        })
    }

    pub async fn connect(&self) -> Result<(), String> {
        match self.test_query().await {
            Ok(_) => Ok(()),
            Err(_) => match self.instance.write() {
                Ok(instance) => {
                    let db_config: Arc<DatabaServerConfig> = self.config.clone();
                    let db_address: String = db_config.address.to_owned();
                    let db_username: &str = &db_config.username.to_owned();
                    let db_password: &str = &db_config.password.to_owned();
                    let db_name: String = db_config.database.to_owned();
                    let db_namespace: String = db_config.namespace.to_owned();
                    match instance.connect::<Ws>(db_address).await {
                        Ok(_) => {
                            if let Err(error) = instance
                                .signin(Root {
                                    username: db_username,
                                    password: db_password,
                                })
                                .await
                            {
                                return Err(format!("[DB_ERROR] connect: {}", error.to_string()));
                            }
                            if let Err(error) = instance.use_ns(db_namespace).await {
                                return Err(format!("[DB_ERROR] connect: {}", error.to_string()));
                            }
                            if let Err(error) = instance.use_db(db_name).await {
                                return Err(format!("[DB_ERROR] connect: {}", error.to_string()));
                            }
                            Ok(())
                        }
                        Err(error) => Err(format!("[DB_ERROR] connect: {}", error.to_string())),
                    }
                }
                Err(error) => Err(format!("db error: {}", error.to_string())),
            },
        }
    }
}

#[cfg(test)]
mod test_db {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_db_connection() {
        let app_config = AppConfig::load(None);
        assert!(app_config.is_ok(), "{:?}", app_config.err());
        let config = app_config.unwrap();
        let database_server_config: Arc<DatabaServerConfig> = Arc::new(config.database_server);
        let db = Db::new(database_server_config).await;
        assert!(db.is_ok(), "{:?}", db.err());
        let db_instance = db.unwrap();
        let client = db_instance.connect().await;
        assert!(client.is_ok(), "{:?}", client.err());
        let client = db_instance.connect().await;
        assert!(client.is_ok(), "{:?}", client.err());
    }
}
