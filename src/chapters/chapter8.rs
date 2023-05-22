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
}