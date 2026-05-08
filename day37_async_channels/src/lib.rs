pub async fn sum_via_mpsc(n:u64)->u64{
    let (tx, mut rx)=tokio::sync::mpsc::channel::<u64>(16);
    let prod=tokio::spawn(async move { for i in 1..=n { tx.send(i).await.unwrap(); } });
    let mut total=0;
    while let Some(v)=rx.recv().await { total += v; if v==n { break; } }
    prod.await.unwrap();
    total
}

#[cfg(test)]
mod tests{ use super::*; #[tokio::test] async fn sum10(){ assert_eq!(sum_via_mpsc(10).await,55); } }
