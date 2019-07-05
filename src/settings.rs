use config::{ConfigError, Config, File, Environment};
use serde_derive::Deserialize;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct Settings  {
    inner: Arc<Inner>,
}

impl Settings {
    pub fn throttle_ms(&self) -> Duration {
        Duration::from_millis(self.inner.ms)
    }

    // TODO: Return the full address instead of port!
    pub fn port(&self) -> u16 {
        self.inner.port
    }
}

#[derive(Debug, Deserialize)]
pub struct Inner {
    /// Throttle Interval
    ms: u64,
    port: u16,
}

impl Settings {
    pub fn parse() -> Result<Self, ConfigError> {
        let mut c = Config::new();
        c.merge(File::with_name("rillrate").required(false))?;
        c.merge(Environment::with_prefix("rillrate"))?;
        c.set_default("ms", 100)?;
        c.set_default("port", 12400)?;
        let inner = Arc::new(c.try_into()?);
        Ok(Settings { inner })
    }
}
