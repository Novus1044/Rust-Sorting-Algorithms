// quicksort is another divide and conquer where we split into sublists again just like mergesort
// except in this case we don't need the additional memory allocations  

fn quicksort( list: &mut [i32] ) -> () {

    let list_length = list.len() - 1;

    quicksort_helper( list, 0, list_length );
}


fn quicksort_helper( list: &mut [i32], begin: usize, end: usize ) -> () {

    if begin >= end {
        return;
    }

    let mut left_index = begin;
    let mut right_index = end;
    let mut pivot_on_right = true;

    // Assume the value at the right_index or list[right_index] is the pivot node, want values smaller than it on the left and those larger on the right
    while left_index != right_index {
     
        // Pivot is on the right, check values from the left to see if a swap needs to be performed 
        if pivot_on_right == true {
            if list[right_index] >= list[left_index] {
                left_index += 1
            }
            else {   
                list.swap( left_index, right_index );
                right_index -= 1;
                pivot_on_right = false
            }
        }
        else {
            if list[left_index] <= list[right_index] {
                right_index -= 1; 
            }
            else {
                list.swap( left_index, right_index );
                left_index += 1;
                pivot_on_right = true;
            }
        }
    }

    let pivot_index = left_index; // could have been right as well, same value

    quicksort_helper( list, begin, pivot_index - 1 ); // pivot_index points to the current pivot, need the one right before it
    quicksort_helper( list, pivot_index + 1, end );   // pivot_index points to the current pivot, need the one right after it
}

fn main() -> () {

    let mut values1 = vec![10,7,2,8,1,3];
    let mut values2 = vec![1,1,1,1,1,1];

    quicksort( &mut values1 );
    quicksort( &mut values2 );

    for x in &values1 {
        println!( "{}", *x );
    }
     
    for y in &values2 {
        println!( "{}", *y );
    }
}



    


            


     



    
