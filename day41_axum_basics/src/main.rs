#[tokio::main]
async fn main(){
    let app=day41_axum_basics::app();
    let listener=tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
