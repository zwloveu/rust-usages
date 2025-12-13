mod data_structure_vec;
pub use data_structure_vec::DataStructureVecModuleFeatureRegister;

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
