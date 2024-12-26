#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=batman").await?.print().await?;
    hc.do_get("/hello2/batman").await?.print().await?;
    Ok(())
}
