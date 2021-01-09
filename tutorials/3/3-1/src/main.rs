fn main_main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadow_main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn spaces_main() {
    let _spaces = "   ";
    let _spaces = _spaces.len();
}

fn main() {
    main_main();
    shadow_main();
    spaces_main();
}
