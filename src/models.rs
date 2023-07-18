use dotenv_codegen::dotenv;

pub struct EnvConfig {
    api_base_url: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            api_base_url: dotenv!("API_BASE_URL").to_owned(),
        }
    }
}

impl EnvConfig {
    pub fn clone_api_base_url(&self) -> String {
        self.api_base_url.clone()
    }
}
