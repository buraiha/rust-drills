pub async fn run()->Vec<&'static str>{
    let a=tokio::spawn(async{ tokio::time::sleep(std::time::Duration::from_millis(50)).await; "a" });
    let b=tokio::spawn(async{ tokio::time::sleep(std::time::Duration::from_millis(10)).await; "b" });
    let (ra,rb)=tokio::join!(a,b);
    vec![ra.unwrap(), rb.unwrap()]
}

#[cfg(test)]
mod tests{ use super::*; #[tokio::test] async fn runs(){ let v=run().await; assert_eq!(v.len(),2); } }
