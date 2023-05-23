use std::fs::File;

pub fn chapter9_main() {
     // demo_file_simple_error(); // Uncomment to see how `panic!()` works
     demo_file_create_file_on_error();
}

fn demo_file_simple_error() {
     let hello_file_result: Result<File, std::io::Error> = File::open("hello.txt");
     let hello_file: File = match hello_file_result {
          Ok(file) => file,
          Err(e) => panic!("Erred as : {:?}", e)
     };
}

fn demo_file_create_file_on_error() { 
     let hello_file_result: Result<File, std::io::Error> = File::open("hello.txt");
     let hello_file: File = match hello_file_result {
          Ok(file) => file,
          Err(e) => match e.kind() {
               std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(e) => panic!("Unable to create new file: {:?}", e)
               },
               other_error => {
                    panic!("Unable to understand: {:?}", other_error)
               }
          }
     };
}