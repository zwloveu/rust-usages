use std::{
    collections::HashMap,
    sync::{
        Arc, Weak,
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::{Duration, Instant},
};

use crate::FeatureRegistry;

#[derive(Default)]
pub struct SmartPointerArcModuleFeatureRegister;

impl FeatureRegistry for SmartPointerArcModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();

        features.insert(
            "module09_smart_pointer_arc_pointer_01_strong_weak_count",
            demonstrate_strong_weak_count,
        );
        features.insert(
            "module09_smart_pointer_arc_pointer_02_mpsc_send_receive",
            demonstrate_send_receive,
        );

        features
    }
}

fn demonstrate_strong_weak_count() {
    let strong_ptr: Arc<str> = Arc::from("test_arc");

    println!("Init Strong Count: {}", Arc::strong_count(&strong_ptr));
    println!("Init Weak Count:   {}", Arc::weak_count(&strong_ptr));

    {
        let strong_ptr: Arc<str> = Arc::clone(&strong_ptr);

        println!(
            "Strong + 1 Strong Count: {}",
            Arc::strong_count(&strong_ptr)
        );
        println!("Strong + 1 Weak Count:   {}", Arc::weak_count(&strong_ptr));
    }

    {
        let weak_ptr: Weak<str> = Arc::downgrade(&strong_ptr);
        println!("Weak + 1 Strong Count: {}", Arc::strong_count(&strong_ptr));
        println!("Weak + 1 Weak Count:   {}", Arc::weak_count(&strong_ptr));

        match weak_ptr.upgrade() {
            Some(temp_arc) => {
                println!("Temp Strong Count: {}", Arc::strong_count(&temp_arc));
                println!("Temp Weak Count:   {}", Arc::weak_count(&temp_arc));

                println!(
                    "Weak + 1 Upgrade Strong Count: {}",
                    Arc::strong_count(&strong_ptr)
                );
                println!(
                    "Weak + 1 Upgrade Weak Count:   {}",
                    Arc::weak_count(&strong_ptr)
                );
            }

            None => {}
        }
    }

    println!("Init Strong Count: {}", Arc::strong_count(&strong_ptr));
    println!("Init Weak Count:   {}", Arc::weak_count(&strong_ptr));
}

fn demonstrate_send_receive() {
    let (tx, rx): (Sender<Arc<str>>, Receiver<Arc<str>>) = mpsc::channel::<Arc<str>>();

    let rx_worker = {
        let start = Instant::now();

        thread::spawn(move || {
            println!(
                "[{}ms] [RxWorker] is running on thread: {:?}",
                start.elapsed().as_millis(),
                thread::current().id()
            );

            while let Ok(task_name) = rx.recv() {
                println!(
                    "[{}ms] [RxWorker] Received: {} | Strong Count {}",
                    start.elapsed().as_millis(),
                    task_name,
                    Arc::strong_count(&task_name)
                );

                //thread::sleep(Duration::from_millis(500));

                println!("[{}ms] [RxWorker] completed", start.elapsed().as_millis());
            }
        })
    };

    let tx_worker1 = {
        let start = Instant::now();
        let tx = tx.clone();

        thread::spawn(move || {
            println!(
                "[{}ms] [TxWorker1] is running on thread: {:?}",
                start.elapsed().as_millis(),
                thread::current().id()
            );

            let name: Arc<str> = Arc::from("thread-tx-1");
            for i in 1..=3 {
                thread::sleep(Duration::from_millis(300));
                let msg: Arc<str> = Arc::from(format!("{}: is generating report {}", name, i));
                tx.send(msg).unwrap();
            }
            println!(
                "[{}ms] [{}] send complete",
                start.elapsed().as_millis(),
                name
            );
        })
    };

    let tx_worker2 = {
        let start = Instant::now();
        let tx = tx.clone();

        thread::spawn(move || {
            println!(
                "[{}ms] [TxWorker2] is running on thread: {:?}",
                start.elapsed().as_millis(),
                thread::current().id()
            );

            let name: Arc<str> = Arc::from("thread-tx-2");
            for i in 1..=3 {
                thread::sleep(Duration::from_millis(200));
                let msg: Arc<str> = Arc::from(format!("{}: is generating report {}", name, i));
                tx.send(msg).unwrap();
            }
            println!(
                "[{}ms] [{}] send complete",
                start.elapsed().as_millis(),
                name
            );
        })
    };

    drop(tx);

    tx_worker1.join().unwrap();
    tx_worker2.join().unwrap();
    rx_worker.join().unwrap();

    println!("[Main] exit");
}
