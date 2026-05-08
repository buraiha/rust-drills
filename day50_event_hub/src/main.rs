#[tokio::main]
async fn main(){
    let app=day50_event_hub::app();
    let listener=tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    println!("listening on http://127.0.0.1:3010");
    axum::serve(listener, app).await.unwrap();
}
