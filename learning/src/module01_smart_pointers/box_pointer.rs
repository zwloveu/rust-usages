use std::collections::HashMap;

use crate::FeatureRegistry;

#[derive(Default)]
pub struct BoxPointerModuleFeatureRegister;

impl FeatureRegistry for BoxPointerModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();
        features.insert(
            "module01_smart_pointers_box_pointer_01_new_deref",
            demonstrate_new_deref,
        );
        features.insert(
            "module01_smart_pointers_box_pointer_02_explicit_deref",
            demonstrate_explict_deref,
        );

        features
    }
}

fn demonstrate_new_deref() {
    let b: Box<i32> = Box::new(5);

    println!("b = {b}");

    println!("Box b address: {:p}", &b);
    println!("Box b value address: {:p}", &*b);

    let box_self_addr: *const Box<i32> = &b as *const Box<i32>;
    let data_addr: *const i32 = &*b as *const i32;
    println!("Box b address: {:p}", box_self_addr);
    println!("Box b value address: {:p}", data_addr);
    unsafe {
        println!("Box Data: {}", *data_addr);
    }

    // data_addr can not point to another data,
    // unless: let mut data_addr: *const i32
    let mut data_addr1: *const i32 = data_addr;
    let b: i32 = 10;
    unsafe {
        println!("Raw Pointer Data Original: {}", *data_addr1);
        // then data_addr1 can point to another data
        data_addr1 = &b as *const i32;
        println!("Raw Pointer Data Pointed to Other: {}", *data_addr1);
    }
}

fn demonstrate_explict_deref() {
    let b: Box<i32> = Box::new(5);

    let b_value: i32 = *b;

    println!("Dereferenced value: {b_value}");
}
