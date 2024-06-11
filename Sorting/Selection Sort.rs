
// Selection Sort
pub fn selection_sort<T: Ord>(values: &mut[T]) {
    let len = values.len();
    for i in 0..len {
        let mut smallest = i;
        for j in (i + 1)..len {
            if values[j] < values[smallest] {
                smallest = j;
            }
        }
        values.swap(i, smallest);
    }
}
fn main() {
    let mut numbers = [4, 2, 3, 1, 5];
    selection_sort(&mut numbers);
    println!("after bubble sort: {:?}", numbers);  // prints: [1, 2, 3, 4, 5]
}