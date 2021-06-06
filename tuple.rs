pub fn tuple(){

    // Creating tuple
    let directions:(f64,f64,&str) = (0.34454,0.55443,"Sakarya");

    // Extract lat, long, city from direction
    let lat = directions.0;
    let long = directions.1;
    let city = directions.2;

    println!("Lat: {} Long: {} city: {} ", lat,long,city);

    // Use distracturing 
    let (lats,longs, citys) = directions;
    println!("Lat: {} Long: {} city: {} ", lats,longs,citys);

    // if you dont want city you can ignore with _underscore
    let (latz,longz,_) = directions;
    // this will get lat and long only

    // Mutate Tuple 
    let mut points:(f64,f64) = (1.0,2.0);
    points.0 = "Hello";
    let (x,y) = points;
    println!("{} {}", x,y);

}