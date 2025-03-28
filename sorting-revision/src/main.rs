fn selection_sort(vec: &Vec<i32>) {
    let mut arr = vec.clone();
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[i] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
    println!("{:?}", arr);
}

fn bubble_sort(vec: &Vec<i32>) {
    let mut arr = vec.clone();
    let mut did_swap = 0;
    for i in (0..arr.len()).rev() {
        for j in 0..i {
            if arr[j] > arr[j + 1] {
                let temp = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = temp;
                did_swap = 1;
            }
        }

        if did_swap == 0 {
            return;
        }
    }

    println!("{:?}", arr);
}

// takes the element and place it in the correct order
fn insertion_sort(vec: &Vec<i32>) {
    let mut arr = vec.clone();
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }

    println!("{:?}", arr);
}

fn main() {
    let arr = vec![13, 46, 24, 52, 20, 9];
    selection_sort(&arr);
    bubble_sort(&arr);
    insertion_sort(&arr);
    println!("Hello, world!");
}
