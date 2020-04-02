fn main () {
    let x = vec![1,2,3];
    println!("Cant use x here : {:?}", x);
    let equal_to_x = move |z| {z == x};
    let y = vec![1,2,3];

    println!("{:?}", equal_to_x(y));
}