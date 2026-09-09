use std::io;
#[test]
fn test() {
    setup(true)
}
fn setup(test: bool) {
    let mut input: String = String::new();
    println!("Welcome to this setup for home-server-ews.");
    println!("Please answer the following questions:");
    print!("Is the service that home-server-ews runs on online 24/7? (yes/no)");
    if test != true {
        io::stdin().read_line(&mut input).expect("Error");
    } else {
        input = "yes"
    }
}
fn main() {
    setup(false);
}
