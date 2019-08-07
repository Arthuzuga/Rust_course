pub fn run() {
    println!("Hello from the print.rs file!!!" );
    println!("Number format: {}", 1);
    println!("Printing multiples arguments: first - {}, second - {}", "Brad", "Velma" );
    println!("Printing multiples ordered arguments: first - {0}, second - {1}, thirdOne - {0}", "Brad", "Velma" );
    println!("Printing variable arguments: {name}", name = "John");
    println!("Printing in differents number types: Binary {:b}, Hex: {:x}, Octal: {:o}", 10,10,10 );
    println!("Placeholder for debug trait {:?}",(12, true, "hello") );
    println!("Maths: 10 + 10 = {}", 10+10 );
    
}