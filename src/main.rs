// our gui

use post::Post;

fn main () {
    let mut mypost = Post::new();

    mypost.add_text("I ate a salad for lunch today");
    assert_eq!("", mypost.content());

    mypost.request_review();
    assert_eq!("", mypost.content());

    mypost.approve();
    assert_eq!("I ate a salad for lunch today", mypost.content());

}