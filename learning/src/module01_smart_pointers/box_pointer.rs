use std::collections::HashMap;

use crate::FeatureRegistry;

#[derive(Default)]
pub struct BoxPointerModuleFeatureRegister;

impl FeatureRegistry for BoxPointerModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();
        features.insert("module01_smart_pointers_box_pointer_new_deref", demonstrate_new_deref);
        features.insert("module01_smart_pointers_box_pointer_explicit_deref", demonstrate_explict_deref);
        
        features
    }
}

fn demonstrate_new_deref() {
    let b: Box<i32> = Box::new(5);

    println!("b = {b}");
}

fn demonstrate_explict_deref()
{
    let b: Box<i32> = Box::new(5);

    let b_value: i32 = *b;

    println!("Dereferenced value: {b_value}");
}