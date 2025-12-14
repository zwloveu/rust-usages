use std::collections::{HashMap, HashSet};

use crate::FeatureRegistry;

#[derive(Default)]
pub struct DataStructureHashSetModuleFeatureRegister;

impl FeatureRegistry for DataStructureHashSetModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();
        features.insert(
            "module02_data_structure_hashset_01_basic",
            demonstrate_hashset_basic,
        );

        features
    }
}

fn demonstrate_hashset_basic() {
    let mut fruits: HashSet<&str> = HashSet::new();

    // HashSet is unordered
    fruits.insert("apple");
    fruits.insert("banana");
    let _ = fruits.insert("banana"); // override the same, return false
    fruits.insert("orange");
    println!("fruits: {:?}", fruits); // check the order multiple times

    println!("iterate through fruits");
    for fruit in &fruits {
        println!("{fruit}");
    }

    println!("delete orange");
    fruits.remove("orange");
    println!("fruits: {:?}", fruits);

    println!("two sets below");
    let set1: HashSet<u32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<u32> = [3, 4, 5, 6].iter().cloned().collect();
    println!("{:?}", set1);
    println!("{:?}", set2);
    let intersection: HashSet<u32> = set1.intersection(&set2).cloned().collect();
    println!("(set1 ∩ set2): {:?}", intersection); // {3, 4}
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("(set1 ∪ set2): {:?}", union); // {1, 2, 3, 4, 5, 6}
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("(set1 - set2): {:?}", difference); // {1, 2}
    let difference1: HashSet<_> = set2.difference(&set1).cloned().collect();
    println!("(set2 - set1): {:?}", difference1); // {5, 6}
}
