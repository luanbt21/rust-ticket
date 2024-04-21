use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=luanbt").await?.print().await?;

    hc.do_get("/index.html").await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "user1",
    //         "pwd": "pass1"
    //     }),
    // );
    // req_login.await?.print().await?;
    //
    // // hc.do_get("/hello2/luanbt").await?.print().await?;
    // //
    // let req_create_ticket = hc.do_post(
    //     "/api/tickets",
    //     json!({
    //     "title": "Ticket 1"
    //     }),
    // );
    // req_create_ticket.await?.print().await?;
    //
    // hc.do_get("/api/tickets").await?.print().await?;
    //
    // hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
