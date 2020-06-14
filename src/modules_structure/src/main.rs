mod f {
    pub mod h {
        pub fn a() {println!("a is working !");}

        pub fn b () {
//            super::super::f::h::b();  // infinite loop, status stack overflow
            super::h::a();
            super::k::kl();
        }
    }

    mod k {
        use crate::f::h;

        pub fn kl() {
            k();
        }

        fn k() {
            h::b();
        }
    }
}

fn main () {
    crate::f::h::a();
    self::f::h::b();
    use crate::f::h;
    h::b(); // infinite loop

}