pub fn arrays(){
    // Arrays in rust should have a type and size and this cannot change 
    let mut years:[i32;3] = [2020,2021,2022];
    years[2] = 2023;
    let first_year = years[0];

    // Arrays can ONLY have one type

    // destructuring arrays 
    let [_,this_year,_] = years;
    let x = 2;
    println!("Years {}",years[x]);
    println!("This year {}", this_year);

    // Looping through years array
    for year in years.iter(){
        println!("Next Year {}", year+1);
    }//
    
}
