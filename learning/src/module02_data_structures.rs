mod data_structure_vec;
pub use data_structure_vec::DataStructureVecModuleFeatureRegister;

mod data_structure_hashmap;
pub use data_structure_hashmap::DataStructureHashMapModuleFeatureRegister;

mod data_structure_hashset;
pub use data_structure_hashset::DataStructureHashSetModuleFeatureRegister;

fn custom_reverse<T: Default>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }

    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;

    while start < end {
        let (left_slice, right_slice): (&mut [T], &mut [T]) = vec.split_at_mut(end);
        let left: &mut T = &mut left_slice[start];
        let right: &mut T = &mut right_slice[0];

        let temp: T = std::mem::take(left);
        *left = std::mem::take(right);
        *right = temp;

        start += 1;
        end -= 1;
    }
}

fn unsafe_custom_reverse<T>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }

    let mut start: usize = 0;
    let mut end: usize = vec.len() - 1;
    let ptr: *mut T = vec.as_mut_ptr();

    unsafe {
        while start < end {
            let left_ptr = ptr.add(start);
            let right_ptr = ptr.add(end);

            let temp: T = std::ptr::read(left_ptr);
            std::ptr::write(left_ptr, std::ptr::read(right_ptr));
            std::ptr::write(right_ptr, temp);

            start += 1;
            end -= 1;
        }
    }
}
