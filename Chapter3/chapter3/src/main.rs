fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (p, q, r) = tup;

    println!("The value of q is: {q}");

    let a = [1, 2, 3, 4, 5];
    another_function();
    another_function_2(5);
    let m = five();
    println!("The value of m is: {m}");
}

fn another_function() {
    println!("Another function.");
}
fn another_function_2(x: i32) {
    println!("The value of x is: {x}");

    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
