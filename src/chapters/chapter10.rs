use std::fmt::Display;

struct NewsArticle {
     headline: String,
     location: String,
     author: String,
     content: String,
     when: String
}

struct Tweet {
     username: String,
     content: String,
     reply: bool,
     retweet: bool,
     when: String
}

trait Summary {
     fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
     fn summarize(&self) -> String {
         format!("News reads - {}, by: {}, at: {}, on: ({})", self.headline, self.author, self.location, self.when)
     }
}

impl Summary for Tweet {
     fn summarize(&self) -> String {
         if self.retweet != true {
          format!("Tweet, by: {}, on: {}", self.username, self.when)
         }
         else {
          format!("ReTweet, by: {}, on: {}", self.username, self.when)
         }
     }
}

struct ImportantExcerpt<'a> {
     part: &'a str
}

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

     // `Traits` - Similar to interfaces in C# or Java
     let news_1 = NewsArticle {
          headline: String::from("Rust being picked by big tech companies"),
          author: String::from("Anurag G"),
          content: String::from("Rust's memory safety has given big tech companies an incentive to choose Rust over C/C++ and also their own proprietary languages"),
          location: String::from("Hyderabad"),
          when: String::from("20/05/2023")
     };
     let news1_summary = news_1.summarize();
     println!("{}", news1_summary);

     let tweet_1 = Tweet {
          content: String::from("Rust seems to be taking the programming world by storm"),
          username: String::from("anuragkrishna_g"),
          reply: false,
          retweet: false,
          when: "28/05/2023".to_string()
     };

     let tweet1_summary = tweet_1.summarize();
     println!("{}", tweet1_summary);

     notify(&news_1);
     notify(&tweet_1);

     notify_generic(&news_1);
     notify_generic(&tweet_1);

     // Lifetimes
     let longest_str = longest_string("Namaste", "Namaskaram");
     println!("{}", longest_str);

     extract_important_excerpt_from_paragraph("This is the first sentence of a paragraph. This is the second".to_string());

     println!("Longest is : {}", longest_with_an_announcement("Pranam", "Pranamam", 45));
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

// Monomorphization ensures that there is no performance overhead due to
// the use of Generics in code.

// Any type that implements a trait can be passed as argument
// This is syntactic sugar for `notify_generic`
fn notify(item: &impl Summary) {
     println!("Breaking News: {}", item.summarize());
}

fn notify_generic<T: Summary>(item: &T) {
     println!("Breaking News: {}", item.summarize());
}

// Return longest string
//  The following function will throw an error as the compiler cannot determine
// if the return type of this function borrows from x OR y
// Also if lifetime is not specific, compiler cannot determine if return type is borrowing
// a value from `x` OR `y` OR some other local reference within the function
// fn longest_string_err(x: &str, y: &str) -> &str {
//      if x.len() > y.len() {
//           x
//      }
//      else {
//           y
//      }
// }

// Longest string with generic lifetime
// Read up on Lifetime Elision. Very important
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
     if x.len() > y.len() {
          x
     }
     else {
          y
     }
}

fn extract_important_excerpt_from_paragraph(input_paragraph: String) {
     let excerpt = input_paragraph.split('.').next().expect("No . found");
     let imp_ex = ImportantExcerpt {
          part: excerpt
     };
     println!("The excerpt is : {}", imp_ex.part);
}

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
where
T : Display {
     println!("Announcing : {}", ann);
     if x.len() > y.len() {
          x
     }
     else {
         y
     }
}