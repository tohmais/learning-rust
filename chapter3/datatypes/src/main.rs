fn main() {
    // tuples: fixed-length, can be multiple types. optionally type-annotated.
    let tup = (500, 6.4, 1);

    // this is called 'destructuring'.
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // this is another way of accessing a variable in a tuple.
    println!("The value of y is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; 

    // this is the same as b = [3, 3, 3, 3, 3]
    let b = [3; 5];

    /* integers: (i32 default)
         Length	    Signed	Unsigned
         8-bit	    i8	    u8
         16-bit	    i16	    u16
         32-bit	    i32	    u32
         64-bit	    i64	    u64
         128-bit	i128	u128
         arch	    isize	usize
    */

    // the compiler can do fixed calculations, 
    // so you can do stuff like x = 3 * 4 * 5
    // and it'll be compiles as x = 60.

    // floating point integers
    // f32: 32-bit
    // f64: 64-bit (default)

    // bools are one byte in size

    let t:bool = a[2] == b[2]; // true
    let f:bool = a[3] == b[3]; // false

    println!("{} != {}", t, f)
}
