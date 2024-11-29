// Function to concatenate two string slices
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new(); // Create a new, empty String
    result.push_str(s1);           // Append the first string slice
    result.push_str(s2);           // Append the second string slice
    result                          // Return the concatenated result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with string slices
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated result
    println!("{}", concatenated_string);
}
