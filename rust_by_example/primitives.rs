
use std::fmt;
use std::mem;

#[derive(Debug)]
struct Matrix(i32, i32, i32, i32);

fn main() {
    let _a_float = 1.0;
    //a_float = 2.3;

    #[warn(unused_assigments)]
    let mut an_integer = 23;
    an_integer = 234;

    //an_integer = false;

    let an_integer = 45u64;

     // Integer addition
     println!("1 + 2 = {}", 1u32 + 2);

     // Integer subtraction
     println!("1 - 2 = {}", 1i32 - 2);
     // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
 
     // Short-circuiting boolean logic
     println!("true AND false is {}", true && false);
     println!("true OR false is {}", true || false);
     println!("NOT true is {}", !true);
 
     // Bitwise operations
     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
     println!("1 << 5 is {}", 1u32 << 5);
     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
 
     // Use underscores to improve readability!
     println!("One million is written as {}", 1_000_000u32);

     let text = "TEXT";
     let mut _text_mut : &str = "text";

     //let tuple = (true, 1f64, "color");
     let tuple = (true, 1u64, "color");
     println!("tuple = {:?}", tuple);
     println!("accessing tuple:\n\t-Attr. name: {2},\n\t-Value: {1:#X},\n\t-Enabled: {0}", tuple.0, tuple.1, tuple. 2);

     let (enabled, mut value, attr_name) = tuple;
     value = 123u64;
     println!("{attr_name} = {value:X} [enabled: {enabled}]");

     let tuples = (
        (true, 1u64, "color"),
        (true, 54u32, "size"),
        (false, 'M', "sex"),
        ("product_name", "T-Short")
     );
    println!("tuples = {:?}", tuples);
    println!("tuples = {:#?}", tuples);
    println!("tuples.0 = {:?}", tuples.0);
    println!("tuples.0.1 = {}", tuples.0.1);

    let index = 1;
    let (enabled_1, value_1, attr_name_1) = tuples.1; //tuples.index$; //tuples.1;
    println!("{attr_name_1} = {value_1:X} [enabled: {enabled_1}]");


    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
        }
    };
    println!("Matrix: \n{}", Matrix(1, 2, 3, 4));

    let xs : [i32; 5] = [1,2,3,4,5];
    let ys : [f64; 3] = [0f64; 3];

    println!("xs = {:?}", xs);
    println!("ys = {:?}", ys);
    println!("&xs[1 .. 4] = {:?}", &xs[1 .. 4]);

    println!("len xs = {}", xs.len());
    println!("size xs = {}", mem::size_of_val(&xs));
    println!("size/elems. xs = {}", mem::size_of_val(&xs) / mem::size_of_val(&xs[0]));

    for i in 1 .. xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}
