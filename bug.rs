fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = &vec[0];
    vec.push(3); //this line causes error
    println!("{}", x);
}