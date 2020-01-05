fn main() {
    l45();
    l46();
    l47();
}

fn print_description(title: &str, description: &str) {
    println!("\n#\n# {}: {}\n#", title, description);
}

fn l45() {
    print_description("Listing 4-5", "a function takes ownership of a variable");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // s1 is made invalid by this statement

    println!("The length of '{}' is {}.", s2, len);

    // since calculate_length takes ownership of s, we return it
    // so the caller can use it again
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }
}

fn l46() {
    print_description("Listing 4-6", "a function borrows a variable using a mutable reference");

    let mut s1 = String::from("hello");

    // pass a reference to keep ownership of s1 in this scope
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // mutable reference!
    change(&mut s1);

    // need to recalculate length now though
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // this function can modify some_string because it has the
    // mut keyword
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // calculate_length is only borrowing s, so we don't
    // need to return it
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn l47() {
    print_description("Listing 4-7", "string slices");

    let s = String::from("hello world");

    let word = first_word(&s);
    // let word = first_word2("literally hello world"); // un-comment this to see a cool error!
    let word_string = first_word2(&s);
    let word_literal = first_word2("literally hello world");
    let word_nospace = first_word2("word");

    // s.clear(); // un-comment this line to see a cool error!

    println!("The first word is: {}", word);
    println!("The first word in the string is: {}", word_string);
    println!("The first word in the string literal is: {}", word_literal);
    println!("The word in the one with no spaces is: {}", word_nospace);

    // first_word only accepts a String reference, whereas first_word2
    // is more flexible, because it can take any string slice (e.g. a string literal)!
    fn first_word(s: &String) -> &str {
        first_word2(s)
    }

    fn first_word2(string: &str) -> &str {
        let bytes = string.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
             if item == b' ' {
                 return &string[0..i];
             }
        }

        string
    }
}
