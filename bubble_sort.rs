fn bubble_sort1( list: &mut Vec<i32> ) -> () {

	let list_length = list.len();

	for _ in 0..list_length {
		for i in 0..list_length - 1 {
			if list[i+1] < list[i] {
				list.swap( i, i+1 );
			}
		}
	}
}

fn bubble_sort2( list: &mut Vec<i32> ) -> () {

	let list_length = list.len();
	let mut swap_performed = false;

	for _ in 0..list_length {
		for i in 0..list_length - 1 {
			if list[i+1] < list[i] {
				list.swap( i, i + 1 );
				swap_performed = true;
			}
		}
		if !swap_performed { 
			return; 
		}
		swap_performed = false;
	}
}
 

fn main() -> () {

	let mut values = vec![1,2,3,4,5,6,7];

	bubble_sort2( &mut values );

	for x in &values {
		println!( "{}", *x );
	}	
}
			 
