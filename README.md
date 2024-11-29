# MultiversXRustBootcamp

Concatenate
-------------
A simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

Steps

1- Create a function called concatenate_strings that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.

2- Inside the concatenate_strings function, create a new String called result. Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.

3- Return the result string from the function.

4- In the main function, create two String variables, string1 and string2, and initialize them with appropriate values.

5- Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices). Store the result in a new variable called concatenated_string.

6- Print the concatenated_string variable to the console.

7- Compile and run the program to ensure it works as expected.

Build, Run and Output

![Ekran görüntüsü 2024-11-29 155928](https://github.com/user-attachments/assets/b9521282-ac8d-4936-8c5e-33fe8387a953)

Calculator
------------
A simple calculator in Rust that can perform basic arithmetic operations using enums and pattern matching.

Steps

1- Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.

2- Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.

3- Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.

4- In the main function, prompt the user to input the first number, the operation to be performed, and the second number.

5- Parse the user input into appropriate variables.

6- Create an Operation enum instance with the parsed input values.

7- Call the calculate function with the created Operation enum instance.

8- Print the result to the console.

9- Compile and run the program to ensure it works as expected.

Build, Run and Output

![Ekran görüntüsü 2024-11-29 152251](https://github.com/user-attachments/assets/acf2930c-9d9c-484f-9995-6fa693ddc364)

Basic Banking System
---------------------
A basic banking system using Traits in Rust. The program will allow users to create accounts, deposit and withdraw money, and view their account balance.

Steps

1- Create a Trait called Account that defines the methods deposit, withdraw, and balance. These methods should take a mutable reference to self as an argument.

2- Implement the Account Trait for a struct called BankAccount. The BankAccount struct should have the fields account_number, holder_name, and balance.

3- In the implementation of the deposit method for BankAccount, add the deposit amount to the balance.

4- In the implementation of the withdraw method for BankAccount, subtract the withdraw amount from the balance.

5- In the implementation of the balance method for BankAccount, return the current balance.

6- In the main function, create two BankAccount instances with different account numbers and holder names.

7- Call the deposit method on one of the accounts, passing in a deposit amount.

8- Call the withdraw method on the other account, passing in a withdraw amount.

9- Call the balance method on both accounts and print the result to the console.

10- Compile and run the program to ensure it works as expected.

Build, Run and Output

![Ekran görüntüsü 2024-11-29 153206](https://github.com/user-attachments/assets/6ec00b4c-46b4-460f-a095-18c104f5277e)

Custom Filtering Function
---------------------------
A custom filtering function in Rust that allows filtering elements from a given collection based on a specific condition. The goal is to implement a beginner-friendly solution that avoids using closures to simplify the understanding of the code.

Steps

1- Create a new Rust project by running the following command in the terminal: cargo new my_project

2- Open the main.rs file in a text editor.

3- Define a struct called FilterCondition with a single field of the desired type for filtering. 

4- Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition. 

5- Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition. 

6- In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.

7- Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.

8- Print the filtered result to the console.

9- Compile and run the program to test its functionality.

Build, Run and Output

![Ekran görüntüsü 2024-11-29 154200](https://github.com/user-attachments/assets/41127acb-7c70-41ce-ad0f-f049e4be2037)

Handle Errors
---------------
Error handling to the basic banking system created in the previous task. The program will handle errors related to depositing and withdrawing money from the bank account.

Steps

1- Modify the deposit method of the Account Trait to return a Result<(), String> instead of (). The Result type will be used to handle errors. The String type will be used to hold the error message.

2- Modify the withdraw method of the Account Trait to return a Result<(), String> instead of (). The Result type will be used to handle errors. The String type will be used to hold the error message.

3- In the implementation of the deposit method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant to indicate failure.

4- In the implementation of the withdraw method for BankAccount, use the Ok(()) variant to indicate success and the Err("Error message".to_string()) variant to indicate failure.

5- In the main function, create two BankAccount instances with different account numbers and holder names.

6- Call the deposit method on one of the accounts, passing in a deposit amount. Handle any errors returned by the deposit method using a match statement.

7- Call the withdraw method on the other account, passing in a withdraw amount. Handle any errors returned by the withdraw method using a match statement.

8- Call the balance method on both accounts and print the result to the console.

9- Compile and run the program to ensure it works as expected.

Build, Run and Output

![Ekran görüntüsü 2024-11-29 154642](https://github.com/user-attachments/assets/e710a71e-9648-40dd-82e9-2d6a7d299a01)

