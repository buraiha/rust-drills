#[tokio::main]
async fn main(){ println!("{}", day37_async_channels::sum_via_mpsc(100).await); }
