// include any necessary external libs and crates for fn() implementation

// Simple Search Function
// Takes an int array input and searches for a target value
// int arr[]
// int target
pub fn search(arr: &[i64], target:i64) {
    let i: i64 = 0;
    for i in 0..arr.len() {
        if arr[i] == target {
            return i;
        }
    }
    return -1;
}