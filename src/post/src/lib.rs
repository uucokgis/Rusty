use std::fmt::Error;
use std::cell::RefCell;
use std::borrow::BorrowMut;

#[cfg(test)]
mod tests {
    /*

    Add a reject method that changes the post’s state from PendingReview back to Draft.
    OK

    Require two calls to approve before the state can be changed to Published.
    OK

    Allow users to add text content only when a post is in the Draft state. Hint: have the state
    object responsible for what might change about the content but not responsible for modifying the Post.
    OK

    */
    use crate::Post;

    #[test]
    fn check_approving_for_only_pending_reviews() {
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
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn print_state(&self) -> &str;

    fn get_state_enum(&self) -> StateTypes;

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error>;

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error>;

    fn reset_approved_count(&self);

    fn increase_approved_count(&self);

    // fn give_history(&self) -> &Vec<Box<dyn State>> {
    //     self.history
    // }
}


pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    history: Vec<StateTypes>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            history: vec![],
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(PendingReview::new()))
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let s = &self.state;
        match s {
            Some(stat) => {
                match stat.get_state_enum() {
                    StateTypes::Draft => {
                        self.content.push_str(text)
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            match s.approve() {
                Ok(t) => {
                    self.state = Some(t);

                    // remember old style
                    // let state_enum = &s.get_state_enum();
                    // self.history.push(s.get_state_enum());
                }

                Err(e) => {
                    println!("Error occured");
                }
            };
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            let result = s.reject();
            match result {
                Ok(t) => {
                    self.state = Some(t);
                }
                Err(e) => {}
            }
        }
    }
    pub fn print_state(&self) -> &str {
        let st = self.state.as_ref();

        let msg = match st {
            Some(s) => {
                s.print_state()
            }
            _ => {
                ""
            }
        };

        msg
    }
}

enum StateTypes {
    PendingReview,
    Draft,
    Published,
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approved_count: RefCell::new(0) })
    }

    fn print_state(&self) -> &str {
        "Draft"
    }

    fn get_state_enum(&self) -> StateTypes {
        StateTypes::Draft
    }

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        panic!("Only reviewing request objects can be aproved !")
    }

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        println!("Draft objects can't be rejected !");
        Ok(self)
    }

    fn reset_approved_count(&self) {
        unimplemented!()
    }

    fn increase_approved_count(&self) {
        unimplemented!()
    }
}

struct PendingReview {
    approved_count: RefCell<u8>  // more than one editor?
}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview {
            approved_count: RefCell::new(0)
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("It is still reviewing");
        self
    }

    fn print_state(&self) -> &str {
        "PendingReview"
    }

    fn get_state_enum(&self) -> StateTypes {
        StateTypes::PendingReview
    }

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        self.increase_approved_count();
        println!("approved count : {}", &self.approved_count.borrow());

        if self.approved_count.borrow().eq(&2) {
            println!("Enough vote approved !");
            Ok(Box::new(Published::new()))
        } else {
            println!("There is no enough approve vote !");
            Ok(self)
        }
    }

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        Ok(Box::new(Draft {}))
    }

    fn reset_approved_count(&self) {
        *self.approved_count.borrow_mut() = 0;
    }

    fn increase_approved_count(&self) {
        *self.approved_count.borrow_mut() += 1;
    }
}

struct Published {}

impl Published {
    fn new() -> Published {
        Published {}
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("It is already reviewed !");
        self
    }

    fn print_state(&self) -> &str {
        "Published"
    }

    fn get_state_enum(&self) -> StateTypes {
        StateTypes::Published
    }

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        println!("It is already approved !");
        Ok(self)
    }

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        panic!("Published objects cannot be rejected !")
    }

    fn reset_approved_count(&self) {
        unimplemented!()
    }

    fn increase_approved_count(&self) {
        unimplemented!()
    }
}