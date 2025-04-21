// One line comment
// Rust URL: https://www.rust-lang.org/

/*
*  Block comment
*
*/


fn main() {
    // Variable
    let mut can_change = "this variable can change";
    let can_not_change = "this variable can not change";

    // Primitive Data

    // Signed Integer
    let a_int: i8 = 1;
    let a_int_suffix = 1i8; // sufix natation
    let a_int_2: i16 = 1;
    let a_int_3: i32 = 1;
    let a_int_4: i64 = 1;
    let a_int_5: i128 = 1;
    let a_uint_6: isize = 1;

    // Unsigned Integer
    let a_uint: u8 = 1;
    let a_uint_2: u16 = 1;
    let a_uint_3: u32 = 1;
    let a_uint_4: u64 = 1;
    let a_uint_5: u128 = 1;
    let a_uint_6: usize = 1;

    // Float
    let my_float: f32 = 1.0;
    let my_float_2: f64 = 1.0;

    // UNICODE Charactes
    let my_char: char = 'a';

    // Booleans
    let a_true: bool = true;
    let a_false: bool = false;

    // Unit (Empty Tuple)
    let my_unit = ();

    // Arrays and Tuples
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_tuple: (i32, f64, u8) = (1, 3.0, 1);

    // Print by terminal "¡Hola, [y el nombre de tu lenguaje]!"
    println!("¡Hola, Rust!");
}