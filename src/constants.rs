use lazy_static::lazy_static;

use crate::models::EnvConfig;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = EnvConfig::default();
}
