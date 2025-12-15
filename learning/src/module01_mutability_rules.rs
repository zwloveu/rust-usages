use std::collections::HashMap;

use crate::FeatureRegistry;

#[derive(Default)]
pub struct MutabilityRulesModuleFeatureRegister;

impl FeatureRegistry for MutabilityRulesModuleFeatureRegister {
    fn get_features(&self) -> HashMap<&'static str, fn()> {
        let mut features: HashMap<&'static str, fn()> = HashMap::new();

        features.insert(
            "module01_mutability_rules_01_basic",
            demonstrate_mutability_rules_basic,
        );
        features.insert(
            "module01_mutability_rules_02_closures",
            demonstrate_mutability_rules_closures,
        );
        features.insert(
            "module01_mutability_rules_03_pointers",
            demonstrate_mutability_rules_pointers,
        );
        features.insert(
            "module01_mutability_rules_04_misunderstandings",
            demonstrate_mutability_rules_misunderstandings,
        );

        features
    }
}

fn demonstrate_mutability_rules_basic() {
    // immutable binding
    let a: i32 = 5;
    let a_addr: *const i32 = &a as *const i32;
    println!("a is {:?}, the address is {:p}", a, a_addr);

    // mutable binding
    let mut b: i32 = 10;
    // b_addr can be changed and can point to other
    let mut b_addr: *mut i32 = &mut b as *mut i32;
    println!("b is {:?}, the address is {:p}", b, b_addr);
    b += 1; // b is 11

    /*
    1, &mut b -> a temporary mutable reference to b:
     b is a mutable binding, create a temporary mutable reference to b
    2, *&mut b -> dereferencing this temporary mutable reference
    3, the whole is: the result of dereferencing the temporary mutable reference to b
    */
    *&mut b += 1; // b is 12

    let mut b1: i32 = 20;
    b1 += 1;
    // b1_addr can not be changed and can not point to other
    let b1_addr: *const i32 = &b1 as *const i32;
    println!("b1 is {:?}, the address is {:p}", b1, b1_addr);
    // b1_addr1 can not be changed but can point to other
    let b1_addr1: &mut i32 = &mut b1 as &mut i32;
    *b1_addr1 += 1; // b1 now is 22

    unsafe {
        *b_addr += 1; // b is 13
        b_addr = &mut b1 as *mut i32; // b_addr points to b1
        println!("b is {:?}, the address is {:p}", b, b_addr);
    }

    // immutable reference
    // b_ref1 is an immutable reference to the i32 varaible b
    let b_ref1: &i32 = &b;
    println!("immutable reference b_ref1 to b is {:?}", b_ref1);

    // mutable reference
    // b_mut_ref2 is a mutable reference to the mutable i32 variable b
    let b_mut_ref2: &mut i32 = &mut b;
    *b_mut_ref2 += 1;
    println!(
        "mutable reference b_mut_ref2 to b, add 1, b is {:?}",
        b_mut_ref2
    );

    // mutable binding to mutable reference
    // b_mut_ref3 is a mutable binding to a mutable reference to the mutable i32 variable b
    let mut b_mut_ref3: &mut i32 = &mut b;
    *b_mut_ref3 += 1;
    println!(
        "mutable reference b_mut_ref3 to b, add 1, b is {:?}",
        b_mut_ref3
    );

    b_mut_ref3 = &mut b1;
    println!("b_mut_ref3 points to b1: {:?}", &b_mut_ref3);
}

fn demonstrate_mutability_rules_closures() {
    println!("test");
}

fn demonstrate_mutability_rules_pointers() {
    println!("test");
}

fn demonstrate_mutability_rules_misunderstandings() {
    println!("test");
}
