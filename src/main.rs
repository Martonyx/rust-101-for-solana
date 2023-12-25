

fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let unsigned: u32 = 10;

    // signed integer
    // i8, i16, i32, i64, i128
    let signed = -10;

    // float is used for decimals
    let float = 0.32;

    // println!("Different numbers => {}, {}, {}", unsigned, signed, float);

    // char is used for single character
    let character = 'a';
    // println!("Character => {}", character);

    // boolean is used for true or false
    let boolean = true;
    // println!("Boolean => {}", boolean);

    // tuple is used for grouping different data types
    let tuple = (1, -2, 3.0, 4, true);
    // println!("Tuple => {:?}", tuple);

    // array is used for grouping same data types
    let array = [1, 2, 3, 4, 5];
    // println!("Array => {:?}", array);

    // string is used for grouping characters
    let string = "Hello World";
    // println!("String => {}", string);

    // vector is used for grouping same data types and it is dynamic
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.push(6);
    // println!("Vector => {:?}", vector);

    // hash map is used for grouping 2 different data types as key value pair
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    // println!("Hash Map => {:?}", hash_map);

    // enums

    enum Color {
        Red,
        Green,
        Blue,
    }

    // hash set is used for grouping same data types

    let mut hash_set = std::collections::HashSet::new();

    hash_set.insert("John Doe");
    hash_set.insert("Jane Doe");

    // println!("Hash Set => {:?}", hash_set);

    // Looping

    for i in 0..10 {
        println!("Looping => {}", i);
    }

    let mut i = 0;
    while i < 10 {
        println!("Looping => {}", i);
        i += 1;
    }

    let mut counter = 0;
    loop {
        println!("Looping...");

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    // Looping over an array
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Array element: {}", element);
    }

    // Looping over an iterator
    let array = [100, 200, 300, 400, 500];
    for (index, value) in array.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }

    // Functions

    // 1. Defining a function
    fn print_greeting() {

        println!("Hello, world!");
        
        }
        
        // Call the function
        
        print_greeting();
    //    2. Defining a function with a single parameter:

    fn print_number(n: i32) {

        println!("The number is {}", n);
        
        }
        
        // Call the function
        
        print_number(5);

    // 3. Defining a function with multiple parameters:

    fn print_numbers(x: i32, y: i32) {

        println!("The numbers are {} and {}", x, y);
        
        }
        
        // Call the function
        
        print_numbers(5, 10);
    
    //    3. Defining a function with a return type:

    fn add(x: i32, y: i32) -> i32 {

        x + y // No semicolon since it is a return expression
        
        }
        
        // Call the function
        
        let sum = add(5, 10);
        
        println!("The sum is {}", sum);

//4. Defining a function with a return type and explicit return statement:

fn subtract(x: i32, y: i32) -> i32 {

    return x - y;
    
    }
    
    // Call the function
    
    let difference = subtract(10, 5);
    
    println!("The difference is {}", difference);





    //Implementation

    //1. Basic Implementation:

    struct Rectangle { width: u32, height: u32,

    }
    
    impl Rectangle { fn area(&self) -> u32 { self.width * self.height }
    
    }
    
    let rect = Rectangle { width: 30, height: 50 };
    
    println!("The area of the rectangle is {}", rect.area());


    // 2. Implementation with more than one method:

    struct Circle {

        radius: f64,
    
    }
    
    impl Circle {
    
        fn area(&self) -> f64 {
    
            3.14 * (self.radius * self.radius)
    
        }
    
        fn perimeter(&self) -> f64 {
    
            2.0 * 3.14 * self.radius
    
        }
    
    }
    
    let circle = Circle { radius: 10.0 };
    
    println!("The area of the circle is {}", circle.area());
    
    println!("The perimeter of the circle is {}", circle.perimeter());

    //3. impl can also be used for implementing traits, which are defined using the trait keyword.

    trait HasArea { fn area(&self) -> f64;

    }

    struct Square { side: f64,

    }

    impl HasArea for Square { fn area(&self) -> f64 { self.side * self.side }

    } 
}
