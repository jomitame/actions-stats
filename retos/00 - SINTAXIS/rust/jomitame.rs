// One line comment
// Rust URL: https://www.rust-lang.org/

/*
*  Block comment
*
*/

use std::collections::{HashMap, HashSet};


fn main() {
    // Variable
    // String slice
    let mut can_change: &str = "this variable can change";
    // To print variable
    // must to be a string literal
    println!("{can_change}");
    let can_not_change: &str = "this variable can not change";
    can_change = "this variable changed";
    println!("{}", can_change);
    println!("{can_not_change}");

    // String
    let my_string: String = String::from("This is other string");
    println!("{my_string}");
    

    // Primitive Data

    // Signed Integer
    let a_int: i8 = 1;
    println!("{a_int}");
    let a_int_suffix = 1i8; // sufix notation
    println!("{a_int_suffix}");
    let a_int_2: i16 = 1;
    println!("{a_int_2}");
    let a_int_3: i32 = 1;
    println!("{a_int_3}");
    let a_int_4: i64 = 1;
    println!("{a_int_4}");
    let a_int_5: i128 = 1;
    println!("{a_int_5}");
    let a_uint_6: isize = 1;
    println!("{a_uint_6}");

    // Unsigned Integer
    let a_uint: u8 = 1;
    println!("{a_uint}");
    let a_uint_2: u16 = 1;
    println!("{a_uint_2}");
    let a_uint_3: u32 = 1;
    println!("{a_uint_3}");
    let a_uint_4: u64 = 1;
    println!("{a_uint_4}");
    let a_uint_5: u128 = 1;
    println!("{a_uint_5}");
    let a_uint_6: usize = 1;
    println!("{a_uint_6}");

    // Float
    let my_float: f32 = 1.0;
    println!("{my_float}");
    let my_float_2: f64 = 1.0;
    println!("{my_float_2}");

    // UNICODE Charactes
    let my_char: char = 'a';
    println!("{my_char}");

    // Booleans
    let a_true: bool = true;
    println!("{a_true}");
    let a_false: bool = false;
    println!("{a_false}");

    // Unit (Empty Tuple)
    let my_unit = ();
    println!("{my_unit:?}"); // special way to print an empty tuple

    // Arrays and Tuples
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{my_array:?}");
    let my_tuple: (i32, f64, u8) = (1, 3.0, 1);
    println!("{my_tuple:?}");

    // Vectors
    let mut my_vector: Vec<&str> = vec!["Jose", "Sandra", "Salma"];
    my_vector.push("Janell");
    println!("{my_vector:?}");
    println!("{:?}", my_vector[1]);

    // Sets
    let mut my_set: HashSet<&str> = vec!["Tapias", "Gaviria"].into_iter().collect();
    my_set.insert("Roa");
    my_set.insert("Tapias"); // This data won't insert because is duplicated
    println!("{my_set:?}");

    // Maps
    let mut my_map: HashMap<&str, i32> = vec![("Ñeña", 8), ("Sandra", 44)].into_iter().collect();
    my_map.insert("Jose", 46);
    println!("{:?}", my_map);

    // Print by terminal "¡Hola, [y el nombre de tu lenguaje]!"
    println!("¡Hola, Rust!");
}
