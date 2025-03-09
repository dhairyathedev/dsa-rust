pub fn largest_element() {
    let vec: Vec<i32> = vec![1, 7, 9, 20, 10, 2];
    let mut tmp = vec[0];

    for n in vec {
        if n > tmp {
            tmp = n;
        }
    }

    println!("The largest element in the array is: {}", tmp);
}

pub fn second_largest_element() {
    let mut vec: Vec<i32> = vec![1, 7, 11, 9, 20, 10, 2];
    //    Brute force approach O(NLogn + N)
    //    vec.sort();
    //    let largest = vec[0];
    //    let mut second_largest = 0;
    //    for n in vec.iter().rev().skip(1) {
    //        if *n != largest {
    //            second_largest = *n;
    //            break;
    //        }
    //    }

    // Better solutions O(2N)
    //let mut largest = vec[0];
    //let mut second_largest = 0;
    //for n in &vec {
    //    if *n > largest {
    //        largest = *n;
    //    }
    //}

    //for n in &vec {
    //    if *n > second_largest && *n != largest {
    //        second_largest = *n;
    //    }
    //}

    // optimal apprach
    let mut largest = vec[0];
    let mut second_largest = -1;
    for i in 1..vec.len() {
        if vec[i] > largest {
            second_largest = largest;
            largest = vec[i];
        } else if vec[i] > largest && vec[i] > second_largest {
            second_largest = vec[i];
        }
    }

    println!(
        "The second largest element in the array is: {}",
        second_largest
    );
}

pub fn check_arr_is_sorted() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 9, 8];

    let mut res: bool = true;

    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            res = false;
        }
    }

    println!("The array sorted status is: {}", res);
}

pub fn remove_duplicate_sorted() {
    let mut vec: Vec<i32> = vec![1, 2, 2, 2, 3, 3];
    vec.sort();

    let mut i: usize = 0;
    for j in 1..vec.len() {
        if vec[j] != vec[i] {
            vec[i + 1] = vec[j];
            i += 1;
        }
    }

    println!("The unique set of the array is: {:?}", &vec[0..i + 1]);
}
