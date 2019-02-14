// data dypes in rust

fn main() {
    arrays();
}

fn arrays () {
    let a = [1,2,3,4,5]; // data type of each must be SAME !
    // unlike tuple, we need to give lenght of array.
    let b = a[1];
    println!("{}", b);
    println!("{}", a.len());
}
fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // zipping (structuring)

    let (x, y, z) = tup;  // unzipping (destructuring)

    let second = tup.1;

    let first = tup.0;


    println!("second : {}", second);
    println!("len : {}", lenght_arr);

}