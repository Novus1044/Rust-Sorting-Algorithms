fn merge_sort( list: &mut Vec<i32> ) -> () {

	let list_length = list.len();

	if list_length == 1 {
		return;
	}

	let midpoint = list_length / 2;

	let mut left_sublist = Vec::with_capacity( midpoint );
	let mut right_sublist = Vec::with_capacity( list_length - midpoint );

	for i in 0..midpoint {
		left_sublist.push( list[i] );
	}

	for i in midpoint..list_length {
		right_sublist.push( list[i] );
	}

	merge_sort( &mut left_sublist );
	merge_sort( &mut right_sublist );

	merge( list, &mut left_sublist, &mut right_sublist );
}

fn merge( list: &mut Vec<i32>, 
		  left_sublist: &mut Vec<i32>, 
		  right_sublist: &mut Vec<i32> ) -> () {

	let mut i: usize = 0;
	let mut left: usize = 0;
	let mut right: usize = 0;

	while left < left_sublist.len() && right < right_sublist.len() {
		
		if left_sublist[left] < right_sublist[right] {
			list[i] = left_sublist[left];
			i += 1;
			left += 1;
		}
		else
		{
			list[i] = right_sublist[right];
			i += 1;
			right += 1;
		}		
	}
	
	while left < left_sublist.len() {
		list[i] = left_sublist[left];
		i += 1;
		left += 1;
	}

	while right < right_sublist.len() {
		list[i] = right_sublist[right];
		i += 1;
		right += 1;
	}
}
	


fn main() -> () {

	let mut array = vec![4,3,9,6,1,7];

	merge_sort( &mut array );

	for x in array {
		println!( "{}", x );
	}
}





