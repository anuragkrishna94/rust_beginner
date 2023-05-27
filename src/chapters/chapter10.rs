pub fn chapter10_main() {
     duplicate_largest_number();
     
     let v1: Vec<u32> = vec![4,52,37,46,3,6, 45,7];
     let max_v1: &u32 = largest_number_module_u32(&v1);
     println!("Largest of {:?} is: {1}", v1, max_v1);

     let v2: Vec<i32> = vec![-4,-52,-37,-46,3,6, 45,7];
     let max_v2: &i32 = largest_number_module_i32(&v2);
     println!("Largest of {:?} is: {1}", v2, max_v2);

     let max_v1_2: &u32 = largest_item_module_generic(&v1);
     println!("Largest of {:?} is: {1}", v1, max_v1_2);

     let max_v2_2: &i32 = largest_item_module_generic(&v2);
     println!("Largest of {:?} is: {1}", v2, max_v2_2);
}

fn duplicate_largest_number() {
     let v: Vec<u32> = vec![3,54,2,75,3,3];
     let mut max_num: &u32 = &v[0];
     for num in &v {
          if num >= max_num {
               max_num = num;
          }
     }
     println!("The largest number of {:?} is : {1}", v, max_num);
}

// &u32 is getting returned without an error as max_num -> &v[0]
// Scope of `max_num` exists in caller of this function as vec will be defined in the caller
fn largest_number_module_u32(v: &[u32]) -> &u32 {
     let mut max_num: &u32 = &v[0];
     for num in v {
          if num >= max_num {
               max_num = num;
          }
     }
     max_num
}

fn largest_number_module_i32(v: &[i32]) -> &i32 {
     let mut max_num: &i32 = &v[0];
     for num in v {
          if num >= max_num {
               max_num = num;
          }
     }
     max_num
}

// The following method implements generics and hence duplication is reduced
// as multiple types can use the same method
// `std::cmp::Partial0rd` trait is needed to restrict `T`
fn largest_item_module_generic<T: std::cmp::PartialOrd>(v: &[T]) -> &T {
     let mut max_item = &v[0];
     for item in v {
          if item >= max_item {
               max_item = item;
          }
     }
     max_item
}