use anyhow::Result;
use pinpet_suffix_generator::{config::AppConfig, utils::load_env, run_server};

#[tokio::main]
async fn main() -> Result<()> {
    // 加载环境变量
    load_env()?;
    
    // 加载配置
    let config = AppConfig::load()
        .map_err(|e| anyhow::anyhow!("配置加载失败: {}", e))?;
    
    // 运行服务器
    run_server(config).await
}
