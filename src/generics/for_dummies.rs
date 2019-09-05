// Under normal conditions
fn normal(a: &str) {
    println!("{:?}", a);
}

// what if we want to run this for any type?
/*
We need to make sure about 2 things: Their data types must be "monomorphizmed".
Other one is body of function must be work with any data type. For example
debug trait has to be implemented into input param, else :? will blow.
*/
// Lets use generic

fn generic<T>(a: T)  {
    println!("{:?}", a);
}

// it did not work. Because we have to implement trait also
fn generic_traited<T> (a: T)
    where T: Debug

{
    println!("{:?}", a);
}

// we would also code like this:
fn generic_traitedv2<T: Debug>(a:T) {
    println!("{:?}", a);
}
