#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()>{
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=batman").await?.print().await?;
    hc.do_get("/hello2/batman").await?.print().await?;
    hc.do_get("/src/main").await?.print().await?;

    // correct login
    hc.do_post("/api/login", json!({
        "username":"demo1",
        "pwd": "welcome"
    })).await?.print().await?;

    // incorrect login
    hc.do_post("/api/login", json!({
        "username":"demo2",
        "pwd": "open"
    })).await?.print().await?;


    Ok(())
}
