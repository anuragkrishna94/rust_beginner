use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
     Int(i32),
     Float(f32),
     Text(String)
}

pub fn chapter8_main() {
     // Create a simple new vector of type i32
     // If a Vector is created with initial values, Rust will infer the type
     let mut v: Vec<i32> = Vec::new();
     let mut v1 = vec![1,2,3];

     // Add elements to vector
     v.push(3);
     v.push(9);
     v.push(23);
     v.push(43);
     v.push(33);

     // Get element by indexing syntax
     let elem_at_3: &i32 = &v[2]; // immutable borrow occurs here
     // v.push(22); // mutable borrow. Uncomment this to see Rust's borrow check in action.
     println!("Third element of 'v' is: {elem_at_3}"); // immutable borrow later used

     // Get element at index using `.get()`
     let elem_at_3_with_get: Option<&i32> = v.get(2);
     match elem_at_3_with_get {
          Some(val) => println!("The third element of 'v' is: {val}"),
          _ => println!("No third element")
     }

     // Iterate over a vector.
     // Immutable references to each element.
     for vec_elem in &v {
          print!("{vec_elem} ");
     }
     println!("");
     // Mutable references to each element
     for vec_elem in &mut v1 {
          // `*` is called dereference operator, using which the value of reference can be obtained.
          *vec_elem += 50;
          print!("{vec_elem} ");
          // v1.push(23); // Uncomment to see borrow check error.
     }

     // enums can be used as a type for vector to iterate through different types of data
     let mut sheet_data = vec![
          SpreadsheetCell::Float(12.4),
          SpreadsheetCell::Int(33),
          SpreadsheetCell::Text(String::from("Yes")),
          SpreadsheetCell::Text(String::from("No"))
     ];

     let popped_elem = sheet_data.pop();
     match popped_elem {
          Some(elem) => println!("{:#?}", elem),
          _ => println!("No element left to pop")
     }

     // push_str takes a string slice as parameter.
     // If `push_str` takes ownership of the argument, then the argument cannot be used anywhere subsequent to `push_str`.
     let mut s1 = String::from("Namaste");
     let s2 = "Everyone";
     s1.push_str(s2);

     println!("'s1' became : {s1} after it got added by 's2' {s2}");
     // Uncomment the following code to see error as `push_str` doesn't take ownership
     // let s3 = String::from(", Wassup");
     // s1.push(s3);

     concetenante_using_plus();
     concatenate_with_format();
     print_string_characters("Namaste".to_string());
     println!("");
     print_string_bytes("Namaste".to_string());

     demo_hash_map();
     demo_hash_map_ownership("Color".to_string(), "Red".to_string());     
}

fn concetenante_using_plus() {
     let s1 = "How ".to_string();
     let s2 = "are you?".to_string();
     let s3 = s1 + &s2;
     println!("Result is : {s3}");
     // println!("{s1}"); // This will cause an error as s1 has already been moved into s3.
}

// `format!()` does not take ownership.
fn concatenate_with_format() {
     let s1 = "How ".to_string();
     let s2 = "are you?".to_string();
     let s3 = format!("{s1}{s2}");
     println!("Result: {s3}");
     println!("s1: {s1}, s2: {s2}");
}

fn print_string_characters(s: String) {
     for c in s.chars() {
          print!("{c}-");
     }
}

fn print_string_bytes(s: String) {
     for bytes in s.bytes() {
          print!("{bytes}-");
     }
}

fn demo_hash_map() {
     // `insert()` returns `None`, when key being inserted is not present
     // `insert()` returns Some(T), when key is present and T is `old value`
     let mut h_map = HashMap::new();
     let ret_1: Option<i32> = h_map.insert(String::from("Pandavas"), 5);
     let ret_2: Option<i32> = h_map.insert(String::from("Kauravas"), 100);

     // `.get()` returns an `Option<&i32>
     // `.get().copied()` returns an `Option<i32>`
     let pandava_score = h_map.get("Pandavas").copied().unwrap_or(0);
     println!("Score of pandavas is : {0}", pandava_score);
}

fn demo_hash_map_ownership(hmap_key: String, hmap_value: String) {
     let mut h_map = HashMap::new();
     h_map.insert(hmap_key, hmap_value);

     // The following lines will cause errors as the values of these variables have been
     // moved into `h_map`
     // println!("key: {hmap_key}, value: {hmap_value}");
}