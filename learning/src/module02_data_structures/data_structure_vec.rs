use std::collections::HashMap;

use crate::{FeatureRegistry, module02_data_structures::custom_reverse};

#[derive(Default)]
pub struct DataStructureVecModuleFeatureRegister;

impl FeatureRegistry for DataStructureVecModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();
        features.insert(
            "module02_data_structure_vec_01_basic",
            demonstrate_vec_basic,
        );

        features
    }
}

fn demonstrate_vec_basic() {
    let mut nums: Vec<_> = Vec::<i32>::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);
    println!("nums of vec: {:?}", nums);
    println!("add more");
    nums.insert(1, 5);
    nums.extend(vec![4, 6]);
    println!("nums of vec: {:?}", nums);
    println!("sort nums");
    nums.sort();
    println!("nums of vec: {:?}", nums);
    println!("reverse nums");
    nums.reverse();
    println!("nums of vec: {:?}", nums);
    println!("custom reverse nums");
    custom_reverse(&mut nums);
    println!("nums of vec: {:?}", nums);

    let fruits: Vec<&str> = vec!["apple", "banana", "cherry"];
    println!("fruits are: {:?}", fruits);
    println!("the second fruit is: {:?}", fruits[1]);
}
