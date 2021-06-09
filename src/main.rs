fn main() {
    // pointer_example();
    illegal_borrow()
}

fn pointer_example() {
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;
    println!("{} {} {} {}", x, y, var1, var2)
}

fn illegal_borrow() {
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // assert_eq!(x, 42);
    x = 42;
    // this is okay, can draw a flow from the value assigned above:
    let y = &x;
    // this establishes a second, mutable flow from x, cannot assign from a borrowed value
    x = 43;
    // this continues the fl
    assert_eq!(*y, 42);
}
