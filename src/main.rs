mod server;
use htn_backend::Config;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // our "killer feature": telemetry
    let config = Config::from_env();
    server::start_server(config).await?;
    Ok(())
}
