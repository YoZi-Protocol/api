use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Setting {
    core: CoreSetting,
    database: DatabaseSetting,
}

impl Setting {
    pub fn core(&self) -> &CoreSetting {
        &self.core
    }

    pub fn database(&self) -> &DatabaseSetting {
        &self.database
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CoreSetting {
    bind: BindSetting,
    log: String,
    workers: usize,
    blocking_threads: usize,
    machine_id: Option<u64>,
}

impl CoreSetting {
    pub fn bind(&self) -> &BindSetting {
        &self.bind
    }

    pub fn log(&self) -> &str {
        &self.log
    }

    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn blocking_threads(&self) -> usize {
        self.blocking_threads
    }

    pub fn machine_id(&self) -> u64 {
        self.machine_id.unwrap_or(0xFFFF)
    }
}

impl Default for CoreSetting {
    fn default() -> Self {
        Self {
            bind: BindSetting::default(),
            log: "info".to_owned(),
            workers: 2,
            blocking_threads: 512,
            machine_id: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BindSetting {
    api: String,
}

impl BindSetting {
    pub fn api(&self) -> &str {
        &self.api
    }
}

impl Default for BindSetting {
    fn default() -> Self {
        Self {
            api: "0.0.0.0:8080".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseSetting {
    uri: String,
}

impl DatabaseSetting {
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

impl Default for DatabaseSetting {
    fn default() -> Self {
        Self {
            uri: "sqlite::memory:".to_owned(),
        }
    }
}
