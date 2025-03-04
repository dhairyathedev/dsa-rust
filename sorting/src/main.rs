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

// real life example (sorting while playing cards)
// Time complexity: O(n^2)
fn insertion_sort(arr: &Vec<i32>) {
    let mut arr = arr.clone();
    let n = arr.len();

    for i in 1..n{
        let curr = arr[i];
        let mut prev = i;

        while prev>0 && arr[prev-1] > curr {
            arr[prev] = arr[prev-1];
            prev-=1;
        }
        arr[prev] = curr;
    }
    println!("Insertion Sort: {:?}", arr);
}

fn merge(arr: &mut Vec<i32>, low: usize, mid: usize, high: usize){
    let mut temp: Vec<i32> = Vec::new();
    // [low..mid]
    // [mid+1...hi]
    let mut left = low;
    let mut right = mid + 1;

    while (left <= mid && right <= high) {
        if (arr[left] <= arr[right]) {
            temp.push(arr[left]);
            left+=1;
        } else {
            temp.push(arr[right]);
            right+=1;
        }
    }

    while(left<=mid){
        temp.push(arr[left]);
        left+=1;
    }

    while right <= high {
        temp.push(arr[right]);
        right+=1;
    }
        arr.splice(low..=high, temp);

}

fn merge_sort(arr: &Vec<i32>, low: usize, high: usize){
    let mut arr = arr.clone();
    
    if low >= high{ 
        println!("Merge Sort: {:?}", arr);
        return};
    
    let mid = (low + high) / 2;

    merge_sort(&arr, low, mid);
    merge_sort(&arr, mid+1, high);

    merge(&mut arr, low, mid, high);
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot_value = arr[low]; // Store the pivot value
    let mut i = low;
    let mut j = high;

    while i < j {
        while arr[i] <= pivot_value && i < high {
            i += 1;
        }

        while arr[j] > pivot_value && j > low {
            j -= 1;
        }

        if i < j {
            arr.swap(i, j);
        }
    }
    arr.swap(low, j);
    j
}

fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let partition_index = partition(arr, low, high);

        // Avoid underflow issues by ensuring proper indices
        if partition_index > 0 {
            quick_sort(arr, low, partition_index - 1);
        }
        quick_sort(arr, partition_index + 1, high);
    }
}

fn main() {
    let arr = vec![3, 9, 1, 4, 2];
    selection_sort(&arr);
    bubble_sort(&arr);
    insertion_sort(&arr);

    let mut tmp = vec![4, 8, 1, 9, 2];
    let tmp_len = tmp.len();
    quick_sort(&mut tmp, 0, tmp_len - 1);
    println!("Quick Sort: {:?}", tmp);
    
    println!("Hello, world!");
}
