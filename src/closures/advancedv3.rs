// closures also can take pointer of variable or ownership or mutable pointer.
fn move_sample() {
    let x = vec![1,2,3];
    let equal_to_x = move |z: Vec<u32>| {z == x};
    println!("Cant use x here : {:?}", x);
    let y = vec![1,2,3];
//    assert_eq!(equal_to_x(y));
}
