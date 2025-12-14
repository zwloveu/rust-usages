use std::collections::HashMap;

use crate::FeatureRegistry;

#[derive(Default)]
pub struct DataStructureHashMapModuleFeatureRegister;

impl FeatureRegistry for DataStructureHashMapModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();
        features.insert(
            "module02_data_structure_hashmap_01_basic",
            demonstrate_hashmap_basic,
        );

        features
    }
}

fn demonstrate_hashmap_basic() {
    let mut user_scores: HashMap<_, _> = HashMap::<&str, u32>::new();

    // HashMap is unordered
    user_scores.insert("Alice", 95);
    user_scores.insert("Bob", 88);
    user_scores.insert("Alice", 98); // same key to override old value
    user_scores.insert("alice", 100); // key is sensitive
    println!("user scores: {:?}", user_scores); //check the order multiple times

    // search score for Alice
    if let Some(alice_score) = user_scores.get("Alice") {
        println!("Alice's score is: {alice_score}");
    }

    println!("interate through user scores");
    for (name, score) in &user_scores {
        println!("{name}: {score}");
    }

    println!("add more user scores");
    let mut extra_scores: HashMap<&str, u32> = HashMap::new();
    extra_scores.insert("Charlie", 75);
    extra_scores.insert("David", 82);
    user_scores.extend(extra_scores);
    println!("user scores: {:?}", user_scores);

    println!("delete David");
    user_scores.remove("David");
    println!("user scores: {:?}", user_scores);
}
