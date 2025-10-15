BlueBird Syntax GuideThis guide provides an overview of the core syntax of the BlueBird programming language.VariablesVariables are immutable by default and are declared with let. Use mut to make them mutable. Type annotations are required.let message: string = "Hello, immutable world!"
let mut counter: int = 0
counter = counter + 1 // This is allowed
FunctionsFunctions are declared with fn. Type annotations for parameters and return values are mandatory.fn add(a: int, b: int) -> int {
    return a + b
}
Control FlowBlueBird uses familiar control flow structures with {} to define scopes.let number = 10

if number > 5 {
    print("Number is greater than 5")
} else {
    print("Number is not greater than 5")
}

let mut i = 0
while i < 3 {
    print(f"Loop iteration: {i}")
    i += 1
}
Ownership and BorrowingTo ensure memory safety, BlueBird uses an ownership model.own: Transfers ownership of a variable.borrow: Creates an immutable reference.mut borrow: Creates a mutable reference.fn take_ownership(data: own list<int>) {
    // 'data' is now owned by this function.
}

fn inspect(data: borrow list<int>) {
    // This function borrows 'data' immutably.
}

let my_data = [1, 2, 3]
inspect(borrow my_data) // OK
take_ownership(own my_data) // Ownership is moved
// inspect(borrow my_data) // Compile-time error! 'my_data' was moved.
