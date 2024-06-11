
//Insertion Sort
pub fn insertion_sort<T: Ord>(values: &mut[T]) {
    let len = values.len();
    for i in 1..len {
        for j in (1..=i).rev() {
            if values[j - 1] <= values[j] {
                break;
            }
            values.swap(j - 1, j);
        }
    }
}

fn main() {
    let mut numbers = [4, 2, 3, 1, 5];
    insertion_sort(&mut numbers);
    println!("after bubble sort: {:?}", numbers);  // prints: [1, 2, 3, 4, 5]
}