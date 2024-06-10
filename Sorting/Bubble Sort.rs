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

// Insertion Sort
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
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);  // prints: [1, 2, 3, 4, 5]
}