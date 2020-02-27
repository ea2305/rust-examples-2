/**
 * Write a program that prints out all the elements of the list that are less than 5.
 * url: https://www.practicepython.org/exercise/2014/02/15/03-list-less-than-ten.html
 */

fn main() {
    // define array and limit of validation
    let limit: usize = 5; // we can pass the value coz is not a heap candidate
    let list: [usize; 11] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    // call print with out index function
    println!("print elements without index -------------------------");
    print_without_index(&list, limit);
    println!("print elements without index -------------------------");
    print_with_index(&list, limit);
}

fn print_without_index (list: &[usize], limit: usize) {
    // start interation of array
    for &item in list.iter() {
        // print and validate
        if item > limit {
            println!("value: {}", item);
        }
    }
}

fn print_with_index (list: &[usize], limit: usize) {
    for (index, &item) in list.iter().enumerate() {
        if item > limit {
            println!("index: {}, value: {}", index, item);
        }
    }
}
