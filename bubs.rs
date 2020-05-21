use std::io;
use std::collections::HashMap;

fn bubs<T: Ord>(arr: &mut[T]) {

    let mut size = arr.len();
    let mut swp_cond = true;
 
    while swp_cond {
        swp_cond = false;
 
        for i in 1..size {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swp_cond = true;
            }
        }
 
        size -= 1;
    }
}

 
fn main() {
    
    let mut arr_size = String::new(); // make a mutable string variable

    println!("Enter the size for array: ");

    io::stdin().read_line(&mut arr_size); //to get input from the user

    let arr_size1 : u64 = arr_size.trim().parse().unwrap();

    println!("The size of array: {}", arr_size1);

    let mut arr_size2 = arr_size1.len();

    let mut vec = vec![0, arr_size];

    for x in arr_size1{
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)
            .ok()
            .expect("Failed to read");
        vec[x] = buf.trim().parse()
            .ok()
            .expect("Integers only!");
    }
    
    for num in vec {
        println!("num = {}", num);
    } 

    println!("\n");

    let mut numbers = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];
    println!("Your Input: {:?\n}", numbers);

 
    bubs(&mut numbers);
    println!("Sorted Array: {:?}", numbers);

    ///*
    println!("\n");
 
    let mut strings = ["SE", "IOTS", "ACS", "OS", "CS"];
    println!("Your Input: {:?}", strings);
 
    bubs(&mut strings);
    println!("Sorted Array: {:?}", strings);

    println!("\n"); 
    //*/
}
