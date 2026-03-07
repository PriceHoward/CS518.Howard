/*
Algorithm Followed: https://www.geeksforgeeks.org/dsa/quick-sort-algorithm/

Issues with quick sort in rust: The way mutability works in rust makes creating a quicksort a slight issue.
Because a mutable object in rust does not allow you to access it more than once at a time. Quicksort requires simultaneously working on two sides of the input array. So to make this work, 
We have to create two different sides of the list in our partition. We do this by callin split_at_mut. This now gives us two mutable
lists so we can do the sort correctly between these two lists.

We also canot use just normal:  
temp_value = arr[i]; 
arr[i] = arr[i+1];
arr[i+1] = temp_value;

In C++ or other languages this if fully allowed. But Rust does not like this due to the mutablility of the array and how you cannot change a mutable value but we can use built in rust commands 
like swap to allows us to safely do this.

Finally there is another issue with the mutablility of rust. The pivot value in rust is normally just stored from the array into a variable. In rust we cannot do that as since the value is mutable as it is apart of the array,
and we are also trying to manipulate the array, We have to use the clone command to be able to take a copy of the value to use as a pivot. In other langguages you do not have to worry about this issue.


*/

pub fn quicksort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_value = partition(arr);
    let (left_of_pivot, right_of_pivot) = arr.split_at_mut(pivot_value);
    quicksort(left_of_pivot);
    quicksort(&mut right_of_pivot[1..]);
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let last = arr.len() - 1;
    let pivot_value = arr[last].clone();

    let mut swap_index: usize = 0;

    for i in 0..last {
        if arr[i] <= pivot_value {
            arr.swap(swap_index, i);
            swap_index += 1;
        }
    }
    arr.swap(swap_index, last);
    swap_index
}

fn main() {
    let mut unsort_vec = Vec::new();
    unsort_vec.push(99);
    unsort_vec.push(18);
    unsort_vec.push(22);
    unsort_vec.push(6);
    unsort_vec.push(75);
    unsort_vec.push(5);
    unsort_vec.push(4);
    unsort_vec.push(11);
    unsort_vec.push(89);
    unsort_vec.push(1);

    for i in &unsort_vec {
        println!("{}", i);
    }

    quicksort(&mut unsort_vec);
    println!("-----------Sorted------------");
    for i in &unsort_vec {
        println!("{}", i);
    }
}
