use std::fmt;

// This is main function.
/* Block comments which go to the closing delimiter. */
/// Generate library docs for the following item.
/// Generate library docs for the following item.
///! Generate library docs for the following item.
///! Generate library docs for the following item.
/* 
* This is another type of comment, a block comment. In general,
* line comments are the recommended comment style. But
* block comments are extremely useful for temporarily disabling
* chunks of code. /* Block comments can be /* nested, */ */
* so it takes only a few keystrokes to comment out everything
* in this main() function. /*/*/* Try it yourself! */*/*/
*/
/*
Comment
*/
fn main() {
    println!("Hello World!");
    
    let no_days = format!("This month has {} days!", 31);
    print!("{}", no_days);

    let new_line = String::from("\n");
    print!("{}", new_line);

    eprintln!("Error NullPointerException on line {1},\nElement NotFound {0}", "line 123", "line 345");

    println!("{subject}\n{verb}\n{value}", subject = "John", verb = "has", value = "31 years");

    println!("Base 10:               {}",   69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C

    let width : usize = 5;
    let number_integer : i32 = 2; 

    println!("Postfix with spaces: [{number:<width$}]", number = 1);
    println!("Prefixes with spaces: [{number_integer:>width$}]");
    println!("Postfixes with 0: {number_float:0<5}", number_float = 23.45);
    println!("Postfixes with 0: {number_float:0<10}", number_float = 23.45);
    println!("Prefixes with 0: {number_float:0>5}", number_float = 23.45);
    println!("Prefixes with 0: {number_float:0>width_zec$}", number_float = 23.45, width_zec = 10);
    println!("Prefixes with 0: {number_float:0>width_zec$.prec$}", number_float = 23.45, width_zec = 10, prec = 5);
    println!("Prec. 3: {number_small:.prec$}", number_small = 0.123456789, prec = 3);
    println!("Prec. 3 (round): {number_small:.prec$}", number_small = 0.12399999999, prec = 3);

    #[allow(dead_code)]
    struct Structure(i32);

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // compile error 
    //println!("UnPrintable(7) = {:?}", UnPrintable(7));

    println!("DebugPrintable(7) = {:?}", DebugPrintable(7));

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("Deep(DebugPrintable(3) = {:?}", Deep(DebugPrintable(3)));
    println!("Deep(DebugPrintable(4) = {:#?}", Deep(DebugPrintable(4)));

    #[derive(Debug)]
    struct Person<'a> {
        name : &'a str,
        age : u8
    };

    let name = "Peter";
    let age = 28;
    let peter = Person {name, age};
    println!("Peter = {:#?}", peter);


    impl fmt::Display for Person<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Persone Name: {}, Persone Age: {}", self.name, self.age)
        }
    };
    println!("Person Perter = {}", peter);
    println!("Person Perter Debug = {:?}", peter);

}
