/**
 * Write a program that takes a list of numbers (for example, a = [5, 10, 15, 20, 25])
 * and makes a new list of only the first and last elements of the given list. For practice, write this code inside a function.
 * URL: https://www.practicepython.org/exercise/2014/04/25/12-list-ends.html
 */
fn main() {
    let list = [5, 10, 15, 20, 25];
    let new_list = get_last_items(&list, 4);

    println!("elements: {:?} vs {:?}", list, new_list);
}

fn get_last_items (items: &[isize], elements: usize) -> &[isize]{
    if elements > items.len() {
        return &[];
    }
    let li = items.len() - elements;
    let ls = items.len();
    &items[li..ls]
}