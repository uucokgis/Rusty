//use art::kinds::PrimaryColor;
//use art::utils::mix;

// or you may type these : (remember re-export)
use new_crate::PrimaryColor;
use new_crate::mix;


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}