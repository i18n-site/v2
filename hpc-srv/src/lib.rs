use aok::Result;
use axum::{body::Bytes, http::HeaderMap, routing::put, Router};
use hpc::CodeBody;

mod autogen;

const BATCH_LIMIT: usize = 64;

#[axum::debug_handler]
pub async fn hpc(headers: HeaderMap, body: Bytes) -> CodeBody {
  hpc::run::<autogen::Hpc, BATCH_LIMIT>(headers, body).await
}

genv::def!(PORT:u16 | 2025);

pub async fn srv() -> Result<()> {
  let port = PORT();
  let router = Router::new();
  let router = axum_layer::layer(router.route("/", put(hpc)));
  hpc::srv(port, router).await
}
