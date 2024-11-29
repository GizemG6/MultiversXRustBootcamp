// Define the FilterCondition struct
struct FilterCondition {
    value: i32, // The desired type for filtering
}

impl FilterCondition {
    // Method to check if an item matches the condition
    fn is_match(&self, item: &i32) -> bool {
        *item > self.value // Example condition: item is greater than the value
    }
}

// Function to filter elements based on a given condition
fn custom_filter(collection: Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut result = Vec::new(); // Create an empty vector to store filtered elements
    for item in collection {
        if condition.is_match(&item) {
            result.push(item);
        }
    }
    result
}

fn main() {
    // Create a collection of integers
    let numbers = vec![10, 20, 30, 40, 50];

    // Define a filter condition
    let condition = FilterCondition { value: 25 };

    // Apply the custom_filter function
    let filtered_numbers = custom_filter(numbers, &condition);

    // Print the filtered result
    println!("Filtered numbers: {:?}", filtered_numbers);
}
