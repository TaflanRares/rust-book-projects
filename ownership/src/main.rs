fn main() {
    let mut s1 = String::from("hello");
    let mut s1 = String::from("ahoy");
    s1.push_str(", world!");

    let s2 =  s1.clone();
    let s2 = String::from("hello, world!");

    let s3 = takes_and_gives_back(s2);
    let s3 = gives_ownership();

    println!("{s1} | {s3}");
}

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}