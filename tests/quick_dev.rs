#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/wawa").await?.print().await?;
    
    // hc.do_get("/src/main.rs").await?.print().await?;

    let reqlog=hc.do_post(
        "/api/login",
        json!({
            "username": "tester",
            "pwd": "account"
        })
    );
    reqlog.await?.print().await?;

    // let reqcreateticket = hc.do_post(
    //     "/api/tickets",
    //     json!({
    //         "title":"Ticket AAA"
    //     })
    // );
    // reqcreateticket.await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;
    
    Ok(())
} //$ cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"