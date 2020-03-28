
// https://tech-blog.optim.co.jp/entry/2019/11/08/163000#Rust-139
// https://qiita.com/kbone/items/7f7847376fac78b0ebcb

use futures::{
    executor::{block_on, ThreadPool, LocalPool, LocalSpawner},
    future,
    future::{ready, pending, FutureExt},
    task::SpawnExt,
    task::LocalSpawnExt
};


async fn long_task1(task_id: usize) {
    println!("  >> Task {} on {:?}", task_id, std::thread::current().id());
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("  >> Task {} done", task_id);
}


async fn long_task2(task_id: usize) -> (usize, String) {
    (task_id, task_id.to_string())
}

async fn long_task3(task_id: usize, sleep_time: u64) {
    for i in 0..sleep_time {
        println!("  >> Task {} {}/{}",task_id, i, sleep_time);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("  >> Task {} done", task_id);
}

async fn long_task4(task_id: usize, sleep_time: u64) {
    println!("  >> Task {} start", task_id);
    std::thread::sleep(std::time::Duration::from_secs(sleep_time));
    println!("  >> Task {} done", task_id);
}

async fn long_task5(task_id: usize) -> usize {
    task_id
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let thread_pool = ThreadPool::new()?;
    let mut local_pool = LocalPool::new();

    println!("1. basic example: {:?} {:?}", block_on(long_task1(0)), block_on(long_task2(1)));

    let mut futures = Vec::new();
    for i in 0..8 {
        futures.push(thread_pool.spawn_with_handle(long_task1(i))?);
    }
    println!("2. async jobs on multi threads {:?}", block_on(future::join_all(futures)));


    let mut futures = Vec::new();
    for i in 0..2 {
        futures.push(thread_pool.spawn_with_handle(long_task2(i))?);
    }
    println!("3. async jobs returning values on multi threads {:?}", block_on(future::join_all(futures)));


    let futures = future::join( 
            thread_pool.spawn_with_handle(long_task1(2))?, 
            thread_pool.spawn_with_handle(long_task2(3))?
        );
    println!("4. join two future returning different type value {:?}", block_on(futures));


    let futures = future::join( 
            thread_pool.spawn_with_handle(long_task3(4, 10))?, 
            thread_pool.spawn_with_handle(long_task3(5, 4))?
        );
    println!("5. join two future returning different sleeping time {:?}", block_on(futures));

    let futures = future::join( 
            long_task3(6, 3), 
            long_task3(7, 2)
        );
    println!("6. join two future, but run sequentially(1:1) {:?}", block_on(futures));

    let futures = future::join( 
            long_task3(6, 4), 
            long_task3(7, 2)
        );
    println!("6. join two future using LocalPool(1:M) {:?}", local_pool.run_until(futures));

    let spawner = local_pool.spawner();
    spawner.spawn_local(long_task4(8, 5)).unwrap();
    spawner.spawn_local(long_task4(9, 2)).unwrap();
    println!("7. seqentially? {:?}", local_pool.run());

    let futures = future::join( 
            spawner.spawn_local_with_handle(long_task3(10,5))?, 
            spawner.spawn_local_with_handle(long_task3(11,4))?
        );
    println!("8. join two future returning different type value {:?}", local_pool.run_until(futures));
    
    println!("9. Map {:?}", block_on(long_task5(10).map(|v| v + 1)));

    Ok(())
}