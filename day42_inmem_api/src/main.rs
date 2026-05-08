#[tokio::main]
async fn main(){
    let app=day42_inmem_api::app();
    let listener=tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("listening on http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}
