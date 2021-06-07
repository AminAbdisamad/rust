mod tuple;
mod structs;
mod arrays;
mod practice_3;
mod enums;
fn main (){
   
    // String Interpolation
    // 1. printlin! 2. format! 3. panic!

    // tuple::tuple();
    // structs::structs();
    // arrays::arrays();

    // Exercises
    practice_3::main();
    enums::main();
   
   

}


fn _multiply(x:f64, y:f64)->f64{
    let name = "hassan";
    if name.starts_with('h'){
        println!("{} starts with (h) ",name)
    }else{
        println!("{} Not start with  (h) ",name)
    }
return x*y;
}

