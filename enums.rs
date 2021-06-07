enum Color {
    Red,
    Green,
    Blue,
    Custom {red:u8,green:u8,blue:u8},
}
// Color is the type and values such as Red,Green and Blue are variants



// Enum Methods 
impl Color{
    // fn rgb(color:Color)->(u8,u8,u8){
        
    // }
       
    
}
// self with lower s and Self with capital S are special values in rust  and they're 
// only used inside an impl and it means give one of these types 

pub fn main(){
    let go:Color = Color::Green;
    let stop:Color = Color::Red;
    let slow_down:Color = Color::Blue;
    let purple:Color = Color::Custom {red:100,green:0,blue:250};
    // println!("Purple Color {}",purple);
    // To get values from enums we use pattern matching
    // PATTERN MATCHING
    let current_color = Color::Blue;

    match current_color {
        Color::Green =>{
            println!("Its Green");
        }
        Color::Blue =>{
            println!("Its Blue");
        }
        Color::Red =>{
            println!("Its red");
        }
        Color::Custom {red,green,blue}=>{
            println!("Custom Colors: {} {} {} ",red,green,blue);
        }

        // Match is similar to switch statement however no break statement is used.

       
    };
    let go_color = match go {
        Color::Blue =>{
            "Its blue"
        }
        Color::Green =>{
            "Its green"
        }
        // catch all pattern
        _=>{
            "NO Idea"
        }

    };
    println!("Go Color {}",go_color);
   
    // Rust does NOT have undifined OR Null

// Type parameters 
let message = "Hello there";
// let last_char = message.pop();

// let email_str: Option<String> = Some("myemail");


}


// Type parameters 
enum Option<T>{
    None,
    Some(T),
}
enum Result<O,E>{
    Ok(O),
    Err(E),
}