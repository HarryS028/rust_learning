pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("May name is {} and I am {}", name, age);
    age = 38;
    println!("May name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) =("Brad", 37);
    println!("{} is {}", my_name, my_age);
}