
// Quick Sort
pub fn quick_sort<T: Ord>(values: &mut[T]) {
    let len = values.len();
    if len < 2 {
        return;
    }
    let pivot_index = partition(values);
    quick_sort(&mut values[0..pivot_index]);
    quick_sort(&mut values[pivot_index + 1..len]);
}

fn partition<T: Ord>(values: &mut[T]) -> usize {
    let len = values.len();
    let pivot_index = len / 2;
    values.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if values[j] <= values[len - 1] {
            values.swap(i, j);
            i += 1;
        }
    }
    values.swap(i, len - 1);
    i
}

fn main() {
    let mut numbers = [4, 2, 3, 1, 5];
    quick_sort(&mut numbers);
    println!("after bubble sort: {:?}", numbers);  // prints: [1, 2, 3, 4, 5]
}