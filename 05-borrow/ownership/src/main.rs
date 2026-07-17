#![allow(dead_code)]
fn main() {
    // exo1();
    exo2();
}

fn exo2() {
    let mut numbers = vec![1, 2, 3, 4, 4, 5, 6];
    let evens = separate_evens_v2(&mut numbers);

    println!("Remaining odds: {:?}", numbers); // Expecting: [1, 3, 5]
    println!("Extracted evens: {:?}", evens);  // Expecting: [2, 4, 6]
}

fn separate_evens(numbers: &mut Vec<u32>) -> Vec<u32> {
    let mut evens = Vec::new();

    let mut i: usize = 0;
    while i < numbers.len() {
        if numbers[i] % 2 == 0 {
            evens.push(numbers[i]);
            numbers.remove(i);
        }
        else {
            i += 1;
        }
    }
    evens
}

fn separate_evens_v2(numbers: &mut Vec<u32>) -> Vec<u32> {
    let evens = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    numbers.retain(|&x| x % 2 != 0);
    evens
}
fn exo1() {
    let words = vec![
        String::from("rust"),
        String::from("borrowing"),
        String::from("ownership"),
        String::from("compiler"),
    ];

    let longest = find_longest(&words);

    println!("The longest word is: {}", longest);
    println!("The original list is still accessible: {:?}", words);
}

 fn find_longest(words: &[String]) -> &String  {
     let mut curr_longest = &words[0];
     for w  in words  {
         if w.len() > curr_longest.len() {
             curr_longest = w;
         }
     }
     curr_longest
 }