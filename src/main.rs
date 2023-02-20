mod server;
use htn_backend::Config;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let config = Config::init();
    server::start_server(config).await?;
    Ok(())
}
