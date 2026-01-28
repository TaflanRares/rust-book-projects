fn main() {
    let s = String::from("hello world");
    let word_index = first_word(&s);
    println!("First word index: {word_index}");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello:{hello}, world:{world}");

    let word_slice = first_slice(&s);
    println!("First word slice: {word_slice}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("Slice of array: {:?}", slice);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}