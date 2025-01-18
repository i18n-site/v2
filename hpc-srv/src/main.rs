use aok::Result;
use hpcsrv::srv;

#[tokio::main]
async fn main() -> Result<()> {
  xboot::init().await?;
  srv().await
}
