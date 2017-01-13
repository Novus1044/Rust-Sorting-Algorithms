// Insertion sort deals with the sublist
// You pick a location and then you check if it is in the sorted sublist
// If not you have to shuffle all those locations
// You store the current element if it needs to be placed somewhere you 

// The loops are slightly subtle since it seems like this would read past our current buffer
// it doesn't. If the list has 5 elements then i will go from [0,4] (note inclusive) and 
// on the final iteration j will go from [3,0] so it is safe to ask for list[j+1] in this circumstance 
fn insertion_sort1( list: &mut [i32] ) -> () {

	for i in 0..list.len() {  
		for j in (0..i).rev() {
			if list[j] >= list[j+1] {
				list.swap(j, j+1);
			}
			else {
				break;
			}
		}
	}
}

fn main() -> () {

	let mut values = vec![4,2,5,3,1];

	insertion_sort1( &mut values );
	
	for x in &values {
		println!( "{}", *x );
	}
}

	
