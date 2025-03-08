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
    //    Brute force approach
    //    vec.sort();
    //    let largest = vec[0];
    //    let mut second_largest = 0;
    //    for n in vec.iter().rev().skip(1) {
    //        if *n != largest {
    //            second_largest = *n;
    //            break;
    //        }
    //    }

    // Better solutions
    let mut largest = vec[0];
    let mut second_largest = 0;
    for n in &vec {
        if *n > largest {
            largest = *n;
        }
    }

    for n in &vec {
        if *n > second_largest && *n != largest {
            second_largest = *n;
        }
    }

    println!(
        "The second largest element in the array is: {}",
        second_largest
    );
}
