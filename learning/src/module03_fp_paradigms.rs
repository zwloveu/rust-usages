use std::collections::HashMap;

use crate::FeatureRegistry;

#[derive(Default)]
pub struct FunctionalProgrammingParadigmsModuleFeatureRegister;

impl FeatureRegistry for FunctionalProgrammingParadigmsModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();

        features.insert(
            "module03_functional_programming_paradigms_01_iterator",
            demonstrate_fp_01_iterator,
        );
        features.insert(
            "module03_functional_programming_paradigms_02_closures",
            demonstrate_fp_02_closures,
        );

        features
    }
}

fn demonstrate_fp_01_iterator() {
    struct Fib {
        curr: u64,
        next: u64,
    }

    impl Fib {
        fn new() -> Self {
            Self { curr: 0, next: 1 }
        }
    }

    impl Iterator for Fib {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let current_val = self.curr;
            let next_val = self.curr.checked_add(self.next);

            match next_val {
                Some(v) => {
                    self.curr = self.next;
                    self.next = v;

                    Some(current_val)
                }

                None => None,
            }
        }
    }

    let first_gt_1k = Fib::new().find(|&f| f > 1000);
    if let Some(v) = first_gt_1k {
        println!("First number > 1000 in Fib SEQ is: {:?}", v);
    }

    let fib_seq: Vec<u64> = Fib::new().take_while(|&f| f < 1000).collect();
    println!("SEQ elements < 1000: ");
    println!("{:?}", fib_seq);
}

fn demonstrate_fp_02_closures() {
    use std::sync::{Arc, mpsc};
    use std::thread;

    type ProducerFactory = Box<dyn Fn(mpsc::Sender<Arc<str>>) -> thread::JoinHandle<()>>;
    type ConsumerFactory = Box<dyn FnOnce(mpsc::Receiver<Arc<str>>) -> thread::JoinHandle<()>>;

    let make_producer_factory =
        move |msg_template: &'static str, count: usize| -> ProducerFactory {
            Box::new(move |tx: mpsc::Sender<Arc<str>>| {
                thread::spawn(move || {
                    for i in 0..count {
                        let msg = Arc::from(format!("{}: {}", msg_template, i));
                        println!("[{}] sent msg: {}", msg_template, msg);
                        tx.send(msg).unwrap();
                    }
                })
            })
        };

    let make_consumer_factory = move |worker_name: &'static str| -> ConsumerFactory {
        Box::new(move |rx: mpsc::Receiver<Arc<str>>| {
            thread::spawn(move || {
                while let Ok(msg) = rx.recv() {
                    println!("[{}] processing: {}", worker_name, msg);
                }
                println!("[{}] exit", worker_name);
            })
        })
    };

    let fast_gen: ProducerFactory = make_producer_factory("fast_gen", 10);
    let slow_gen = make_producer_factory("slow_gen", 5);
    let consumer_gen = make_consumer_factory("Data_Processor");

    let (tx, rx) = mpsc::channel();

    let tx_handle1 = fast_gen(tx.clone());
    let tx_handle2 = slow_gen(tx.clone());
    let rx_handle1 = consumer_gen(rx);

    drop(tx);

    tx_handle1.join().unwrap();
    tx_handle2.join().unwrap();
    rx_handle1.join().unwrap();
}
