fn main() {
    // borrow
    println!("--------------------------------------------------");
    println!("Borrow example");
    let borrow_string = String::from("borrow text");
    let borrow_string = borrow_function(borrow_string);

    println!("origin text: {}", borrow_string);
    println!("--------------------------------------------------");


    // reference
    println!("--------------------------------------------------");
    println!("Reference example");
    let reference_string = String::from("reference text");
    reference_function(&reference_string);
    println!("original text: {}", reference_string);
    println!("--------------------------------------------------");

    // modify reference strings
    println!("--------------------------------------------------");
    println!("Reference mutation example");
    let mut reference_mutation_string = String::from("reference test mutation");
    reference_mutation_function(&mut reference_mutation_string);

    println!("original text: {}", reference_mutation_string);
    println!("--------------------------------------------------");
}

fn borrow_function (s: String) -> String{
    println!("text: {}", s);
    String::from("fake text")
}

fn reference_function (s: &String) {
    println!("{}", s);
}

fn reference_mutation_function (s: &mut String) {
    s.push_str(" _mod_");
}

fn get_end_word (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if &item == b' ' {
            return i
        }
    }

    s.len()
}