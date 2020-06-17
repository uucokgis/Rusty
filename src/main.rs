use post::Post;

fn main() {
    let mut p = Post::new();
    assert_eq!(p.print_state(), "Draft");

    p.add_text("my post");
    assert_eq!(p.print_state(), "Draft");

    p.request_review();
    assert_eq!(p.print_state(), "PendingReview");

    // p.reject();
    // println!("state of the post : {}", p.print_state());

    p.approve();
    assert_ne!(p.print_state(), "Published");

    p.approve();
    assert_eq!(p.print_state(), "Published");

    println!("state of the post : {}", p.print_state());

    // p.reject();
    // println!("state of the post : {}", p.print_state());
}