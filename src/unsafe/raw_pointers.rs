fn sample_one() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &num as *const i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        // vohaa my first unsafe code !
        println!("value of r1 is : {}", *r1);
        println!("value of r2 is : {}", *r2);

        println!("value of r3 is : {}", *r3);
    }
}

fn sample_two() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let r = &mut v;
    let mid = 3;

    let k = split_at_mut(r, mid);

    println!("destructured k : {:?}, {:?}", k.0, k.1);
}

fn split_at_mut(kesit: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    let len = kesit.len();
    let ptr = kesit.as_mut_ptr();

    assert!(middle <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, middle),
            slice::from_raw_parts_mut(ptr.add(middle), len - middle)
        )
    }
}