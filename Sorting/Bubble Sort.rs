// Bubble Sort
pub fn bubble_sort<T: Ord>(values: &mut[T]) {
    let len = values.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = [4, 2, 3, 1, 5];
    bubble_sort(&mut numbers);
    println!("after bubble sort: {:?}", numbers);  // prints: [1, 2, 3, 4, 5]
}