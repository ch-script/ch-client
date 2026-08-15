#[derive(Debug)]
pub struct SystemProfile {
    pub os: String,
    pub kernel: String,
    pub pkg_manager: String,
    pub init_system: String,
    pub gpu: Option<String>,
    pub wm_de: Option<String>,
    pub core_utils: String,
    pub user_manager: String,
    pub net_manager: String,
}