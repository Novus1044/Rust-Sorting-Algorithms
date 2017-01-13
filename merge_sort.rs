// things to keep in mind:
// mem::uninitialized() actually initializes previously unitialized data
//  -- used with types that have move semantics -- since the type would be dropped

// Arrays are not generic over the size N, so [Foo; 3] and [Bar; 3] are instances
// of the same generic type [T; 3] but [Foo; 3] and [Foo; 5] are entirely different types
//  -- implication: only arrays up to size 32 have trait implementations, auto-converted
//     to slices and the like
//  -- so if you have an array [i32; 3] then you can do &array and it converts to a slice
//  -- otherwise you will get type does not implement <core::iter::Iterator> or something
//     like that
//  a way around this is to use the RangeFull syntax which is the .. which used in 
//  an array creates an array slice of the entire array that you can then iterate through

// clone is a deep copy and copy is a shallow copy
// can clone something that is copy and it just copies, clone is just a more 
// complicated version essentially 


// phantom type parameter is one that doesn't show up at runtime, but is checked
// statically and only at compile time

// data types can use extra generic type parameters as markers or to perform
// type checking at compile time. they have no storage and no runtime behavior

/*****************************
// The Main traits to learn:
// Arithmetic: Add, Sub, Div, Mul, AddAssign, SubAssign, DivAssign, MulAssign
// Shl, Shr, ShlAssign, ShrAssign, BitAnd, BitOr, BitXor, -- and the assign versions
// Drop, Index, IndexMut, Deref, Borrow, AsRef, AsMutRef, Iterator, IntoIterator
// Fn, FnMut, FnOnce (these depend on the contained types that is what makes it Fn, FnMut, FnOnce, not how it accepts an argument) 
*****************************/

// plain function type is fn(params) -> return_type (less flexible then closure)
//  -- if a function can take either a function (through a function pointer) or
//  -- a closure just have it be more flexible and take the closure
// three closure types: FnOnce, FnMut, Fn in increasing strictness


// Fn -- normal
// FnMut -- normal
// FnOnce -- There are some unusual things at play here so the FnBox type is currently needed, and is unstable
//           This is expected to change in the future

// The move keyword signs that we should capture all values by value, meaning that we will take ownership
// of types that are moved normally

// Examples within the standard library
// Iterator::any, so i guess that is the last case is if they belong to a trait
// Iterator::any(), MyType::any() they are associated with the type though so not surprised they are similar

/*************
pub trait Iterator {
	// The type being iterated over
	type Item;

	// `find` takes &mut self` meaning the caller may be borrowed and modified but not consumed
	fn find<'a,P>( self: &'a mut Self, predicate: P ) -> Option<Self::Item>
		where P: FnMut( &Self::Item ) -> bool { // it takes the closure elements by reference

		// details disregarded
	}
}
**************/

// I think ownership finally makes sense to me
// Two things, the pointer needs to point to valid memory
// Know the two catches with read and write:
//  write doesn't free it's destination
//  read must be followed by a write (copy, copy_overlapping, write, write_volatile etc)

// Need function declarations:
//  	pub unsafe fn read<T>(src: *const T) -> T         // doesn't prevent the use of the location so a *src is still initialized so *src would first try to drop
//		pub unsafe fn write<T>(dst: *mut T, src: T) -> T  // the location not the data that is dropped 

// read, write, copy_nonoverlapping, copy, allocate, deallocate, offset
// those are the only 7 functions you need to concern yourself with to write unsafe code in Rust

// so on a write we consume the value and move it into the other variable 
// it basically just drops whatever is in that object
// it says it is initialized, drop that value 
// that is a simple way to leak resources though, not too disimiliar from standard C just a little different looking syntax 
// but this gives you the flexibility to move back into safe rust and do whatever you need to do there

// if something is initialized and it is assigned to again, even if a raw pointer then that value would be dropped

#![feature(alloc_system)]

// questions to ask:
//  can i solve this in place? -- as you are reading in from stdin or something
//  can i store the whole thing in a single data struture -- array or vec
//  should i store parts in two different data structures (of the same type) -- two vecs
//  do i need to use two different data structures -- a vec and a stack

// Merge Sort -- divide and conquer break down a list into several sublist
//            -- until each sublits consists of a single element then merging

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





