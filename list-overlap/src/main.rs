/**
 * Write a program that returns a list that contains only the elements that are common between the lists (without duplicates). Make sure your program works on two lists of different sizes.
 * url: https://www.practicepython.org/exercise/2014/03/05/05-list-overlap.html
 */
fn main() {
    // define two list
    let list_one = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let list_two = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    // display a message to user
    println!("the two list contains the same elements:");

    // init function to get overlap
    let overlap_list = get_overlap(&list_one, &list_two);

    // display the result
    println!("result: {:?}", overlap_list);
}

fn get_overlap (l1: &[usize], l2: &[usize]) -> Vec<usize> {
    // create new empty vect
    let mut result = vec![];

    // iterate throught the array 1
    for i in 0..l1.len() {
        // iterate throught the array 2
        for j in 0..l2.len() {

            // validate, if elemnents are the same, try to insert
            if l1[i] == l2[j] {
                // find if element is already in the list of results.
                let is_added = result.iter().find(|&&x| x == l1[i]);
                // if not, add it
                if is_added == None {
                    result.push(l1[i]);
                } // in other case, ignore
            }
        }
    }
    // return vect of results.
    result
}