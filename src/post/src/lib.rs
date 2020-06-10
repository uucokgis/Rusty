/*
todo: list

    * Add a reject method that changes the post’s state from PendingReview back to Draft.
    * Require two calls to approve before the state can be changed to Published.
    * Allow users to add text content only when a post is in the Draft state. Hint: have the state
    object responsible for what might change about the content but not responsible for modifying the Post.
*/

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
       Post {
           state: Some(Box::new(Draft { })),
           content: String::new()
       }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self ) {
        match self.state.take() {
            Some(s) => {
                self.state = Some(s.request_review())
            }
            _ => self.state = None  // i am not sure about this
        }
//        if let Some(a) = self.state.take() {
//            self.state = Some(a.request_review())
//        }
    }

    pub fn approve(&mut self) {
        match self.state.take() {
            Some(s) => {
                self.state = Some(s.approve())
            }
            _ => self.state = None
        }

//        if let Some(s) = self.state.take() {
//            self.state = Some(s.approve())
//        }
    }

    pub fn reject(&mut self) {
        println!("We rejected your Post request !");
        *self = Post::new();
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
//    fn reject(self: Box<Self>) -> Box<dyn State>;  // i added
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft { }
struct PendingReview { }
struct Published { }

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
//    fn reject(self: Box<Self>) -> Box<State> {  // i added
//        println!("Rejected !");
//        Box::new(Draft{})
//    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}