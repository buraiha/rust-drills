fn main(){ let pool=day35_threadpool::ThreadPool::new(4); for i in 0..8 { pool.execute(move || println!("job {i}")); } }
