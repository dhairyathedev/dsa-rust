// Time complexity: O(n^2)
fn selection_sort(arr: &Vec<i32>) {
    let mut arr = arr.clone();
    let n = arr.len();
    for i in 0..n - 2 {
        let mut min = i;
        for j in i..=n - 1 {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(min, i);
    }
    println!("Selection Sort: {:?}", arr);
}

// Time complexity: O(n^2)
fn bubble_sort(arr: &Vec<i32>) {
    let mut arr = arr.clone();
    let n = arr.len();

    for i in 0..n - 1 {
        // optimization for bubble sort for sorted array
        let mut is_swap: bool = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                is_swap = true;
            }
        }

        if !is_swap { // array is already sorted
            println!("Bubble Sort: {:?}", arr);
            return;
        }
    }

    println!("Bubble Sort: {:?}", arr);
}

fn insertion_sort(arr: &Vec<i32>) {}

fn main() {
    let arr = vec![3, 9, 1, 4, 2];
    selection_sort(&arr);
    bubble_sort(&arr);
    println!("Hello, world!");
}
