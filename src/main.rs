use post::Post;

fn main() {
    let mut p = Post::new();
    p.add_text("my post");
    println!("state of the post : {}", p.print_state());

    p.request_review();
    println!("state of the post : {}", p.print_state());

    p.reject();
    println!("state of the post : {}", p.print_state());

    p.approve();
    println!("state of the post : {}", p.print_state());

    // p.reject();
    // println!("state of the post : {}", p.print_state());
}