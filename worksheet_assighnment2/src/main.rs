// // // // // // // //             // Question 1


            
// // // // // // // // struct Person {
// // // // // // // //     name: String,
// // // // // // // //     age: u32,
// // // // // // // // }

// // // // // // // // fn main() {
// // // // // // // //     let person = Person {
// // // // // // // //         name: String::from("Muhammad Muneeb"),
// // // // // // // //         age: 30,
// // // // // // // //     };

    
// // // // // // // //     println!("Name: {}", person.name);
// // // // // // // //     println!("Age: {}", person.age);
// // // // // // // // }


// // // // // // // //             Question 2


// // // // // // // enum Color {
// // // // // // //     Red,
// // // // // // //     Green,
// // // // // // //     Blue,
// // // // // // // }


// // // // // // // fn get_rgb(color: Color) -> (u8, u8, u8) {
// // // // // // //     match color {
// // // // // // //         Color::Red => (255, 0, 0),
// // // // // // //         Color::Green => (0, 255, 0),
// // // // // // //         Color::Blue => (0, 0, 255),
// // // // // // //     }
// // // // // // // }

// // // // // // // fn main() {



// // // // // // //     let red = Color::Red;
// // // // // // //     let green = Color::Green;
// // // // // // //     let blue = Color::Blue;



// // // // // // //     let rgb_red = get_rgb(red);
// // // // // // //     let rgb_green = get_rgb(green);
// // // // // // //     let rgb_blue = get_rgb(blue);



// // // // // // //     println!("Red: {:?}", rgb_red);
// // // // // // //     println!("Green: {:?}", rgb_green);
// // // // // // //     println!("Blue: {:?}", rgb_blue);
// // // // // // // }


// // // // // //             // Question 3

// // // // // //             fn sum_of_elements(tuple: (i32, i32)) -> i32 {
// // // // // //                 let (first, second) = tuple;
// // // // // //                 first + second
// // // // // //             }
            
// // // // // //             fn main() {
// // // // // //                 let my_tuple = (5, 7);
// // // // // //                 let result = sum_of_elements(my_tuple);
// // // // // //                 println!("Sum of the  elements: {}", result);
// // // // // //             }


// // // // // //             Qestion 4


// // // // // enum OptionValue {
// // // // //     StringValue(String),
// // // // //     NumberValue(i32),
// // // // // }

// // // // // fn print_number_value(option: OptionValue) {
// // // // //     match option {
// // // // //         OptionValue::NumberValue(num) => {
// // // // //             println!("The number is: {}", num);
// // // // //         }
// // // // //         _ => {
// // // // //             println!("The value is not a number.");
// // // // //         }
// // // // //     }
// // // // // }

// // // // // fn main() {
// // // // //     let value1 = OptionValue::StringValue(String::from("Hello, Rust programmers"));
// // // // //     let value2 = OptionValue::NumberValue(42);

// // // // //     print_number_value(value1);
// // // // //     print_number_value(value2);
// // // // // }



// // // // //             Question 5


            

// // // // fn append_string(input_string: &mut String) {
// // // //     input_string.push_str(" World!");
// // // // }

// // // // fn main() {
// // // //     let mut my_string = String::from("Hello");
    
// // // //    println!("Before appending {}", my_string);


// // // //     append_string(&mut my_string);

// // // //     println!("After appending {}", my_string); 
// // // // }



// // // //             Question 6

            


// // // struct Book {
// // //     title: String,
// // // }

// // // impl Book {

// // //     fn new(title: String) -> Book {
// // //         Book { title }
// // //     }

    
// // //     fn get_title(&self) -> &String {
// // //         &self.title
// // //     }
// // // }

// // // fn main() {
    
// // //     let book = Book::new(String::from("The programming book"));

    
// // //     let title_reference = book.get_title();
// // //     println!("Title: {}", title_reference);
// // // }




// //         // Question 8   (a)


// //         // struct Book {
// //         //     title: String,
// //         //     author: String,
// //         //     pages: u32,
// //         // }
        
// //         // fn print_author_name(book: &Book) {
// //         //     match &book.author {
// //         //         author_name => println!("Author's name: {}", author_name),
// //         //     }
// //         // }
        
// //         // fn main() {
// //         //     let my_book = Book {
// //         //         title: String::from("Good Bad Habits"),
// //         //         author: String::from("Muneeb"),
// //         //         pages: 200,
// //         //     };
        
// //         //     print_author_name(&my_book);
// //         // }




// //         // Question 8 (b)

        
// //         fn print_option_description(option_value: Option<i32>) {
// //             match option_value {
// //                 Some(_) => println!("Has a value"),
// //                 None => println!("No value"),
// //             }
// //         }
        
// //         fn main() {
// //             let some_value: Option<i32> = Some(42);
// //             let none_value: Option<i32> = None;
        
// //             print_option_description(some_value);
// //             print_option_description(none_value);
// //         }
        


//             Question 10





use worksheet_assighnment2::Book;
use worksheet_assighnment2::utils;

fn main() {
    let my_book = Book {
        title: String::from("Good or Bad ha bits"),
        author: String::from("Munee b"),
        pages: 200,
    };

    
    utils::display_book(&my_book);
}

