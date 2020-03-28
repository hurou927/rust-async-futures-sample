
use std::time::Duration;

async fn long_task1(task_id: usize) {
    println!("  >> Task {} on {:?}", task_id, std::thread::current().id());
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("  >> Task {} done", task_id);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        tokio::spawn(long_task1(3));
        Ok(())
    })
}
