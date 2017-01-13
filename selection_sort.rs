fn selection_sort( list: &mut [i32] ) -> () {

    for i in 0..list.len() {
        let mut min: usize = i;
        for j in (i+1)..list.len() {
            if list[j] < list[min] { 
                min = j;
            }
        }
        list.swap(i,min); 
    }
}

fn main() -> () {

    let mut values1 = vec![4,2,1,3,7];
    let mut values2 = vec![2,2,2,2,2];

    selection_sort( &mut values1 );
    selection_sort( &mut values2 );
   
    for x in &values1 {
        println!( "{}", x );
    } 
        
    for y in &values2 {
        println!( "{}", y );
    }
}
