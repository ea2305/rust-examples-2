/**
 * Write a program (function!) that takes a list and returns a new list that contains all the elements of the first list minus all the duplicates.
 */

fn main() {
    let mut original: Vec<i32> = vec![1,1,2,3,45,6,78,6,78,9];

    let results: Vec<i32> = remove_duplicates(&mut original);
    println!("{:?}", results);
}

fn remove_duplicates (list: &mut Vec<i32>) -> Vec<i32> {
    let mut new_list: Vec<i32> = vec![];
    for &number in list.iter() {
        let results = new_list.iter().find(|&&e| e == number);
        if results != Some(&number) {
            new_list.push(number);
        }
    }

    new_list
}
