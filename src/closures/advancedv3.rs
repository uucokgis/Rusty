// closures also can take pointer of variable or ownership or mutable pointer.
fn move_sample() {
    let x = vec![1,2,3];
    println!("Cant use x here : {:?}", x);
    let equal_to_x = move |z| {z == x};
    let y = vec![1,2,3];

    println!("{:?}", equal_to_x(y));
}
