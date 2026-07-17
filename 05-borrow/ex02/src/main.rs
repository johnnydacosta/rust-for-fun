fn get_full_name_lifetime<'a>(first: &'a str, last: &'a str) -> String {
    let full_name = format!("{} {}", first, last);

    // We want to return a reference to the combined string
    full_name // we tranfer ownership of full_name to the fn caller
}

fn get_full_name_buff<'a>(first: &'a str, last: &'a str, full_name: &'a mut String) -> &'a mut String {
    full_name.push_str(&format!("{} {}", first, last));
    full_name
}

fn main() {
    let first = "Alice";
    let last = "Smith";
    let mut full_name = String::from("");
    let full_name_ref = &full_name;
    let full_name_copy = full_name.clone();

    let name = get_full_name_buff(first, last, &mut full_name);
    println!("Full name: {}", name);
    let name2 = get_full_name_lifetime(first, last);
    println!("Full name2: {}", name2);

    println!("Full name: {}", full_name);
    println!("Full name copy: {}", full_name_copy);

    // Borrowing a reference to a mutable variable is not allowed
    // println!("Full name ref: {}", full_name_ref);
}