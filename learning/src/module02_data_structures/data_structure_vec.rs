use std::collections::HashMap;

use crate::FeatureRegistry;

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
    super::custom_reverse(&mut nums);
    println!("nums of vec: {:?}", nums);
    println!("unsafe custom reverse nums");
    super::unsafe_custom_reverse(&mut nums);
    println!("nums of vec: {:?}", nums);
    println!("remove the last from nums");
    let _ = nums.pop();
    println!("nums of vec: {:?}", nums);
    println!("change the last");
    if let Some(last) = nums.last_mut() {
        *last = -10000;
    }
    println!("nums of vec: {:?}", nums);
    println!("get subset of nums by slice: from the first to the third");
    println!("slice of nums: {:?}", &nums[0..=2]);

    let fruits: Vec<&str> = vec!["apple", "banana", "cherry"];
    println!("fruits are: {:?}", fruits);
    println!("the second fruit is: {:?}", fruits[1]);
}
