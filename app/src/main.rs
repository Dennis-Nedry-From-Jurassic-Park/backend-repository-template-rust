use module1::render;
use module2::log;
use std::{thread::sleep, time::Duration};

use lever::prelude::*;
use std::sync::Arc;
use lever::txn::prelude::*;


fn main() {
    render();
    log();
    sleep(Duration::from_millis(1000));

    let lotable: Arc<LOTable<String, u64>> = Arc::new(LOTable::new());

    let manager = lever::lever().manager();

    let txn = manager.txn_build(
        // Select concurrency scheme
        TransactionConcurrency::Optimistic,
        // Select isolation scheme
        TransactionIsolation::RepeatableRead,
        // Give timeout for transaction conflict resolution in milliseconds
        100_usize,
        // Work element size inside the given transaction
        1_usize,
        // Name of the transaction
        "basic_txn".into(),
    );

    // RW from 1_000 threads concurrently.
    let thread_count = 1_000;
    let mut threads = vec![];

    for thread_no in 0..thread_count {
        let lotable = lotable.clone();

        let t = std::thread::Builder::new()
            .name(format!("t_{}", thread_no))
            .spawn(move || {
                let key = format!("{}", thread_no);
                lotable.insert(key.clone(), thread_no);
                let _ = lotable.get(&key).unwrap();
            })
            .unwrap();

        threads.push(t);
    }

    for t in threads.into_iter() {
        t.join().unwrap();
    }

    let mut customers = TVar::new(123_456);

    txn.begin(|t| {
        let mut churned = t.read(&customers);
        churned += 1;
        t.write(&mut customers, churned);
    });

    println!("I have {} customers right now. I gained 1.", customers.get_data());
}