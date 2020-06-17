use std::fmt::Error;

#[cfg(test)]
mod tests {
    use crate::Post;

    #[test]
    fn check_approving_for_only_pending_reviews() {
        let mut p = Post::new();
        p.add_text("my post");
        println!("state of the post : {}", p.print_state());

        p.request_review();
        println!("state of the post : {}", p.print_state());

        p.approve();
        println!("state of the post : {}", p.print_state());
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn print_state(&self) -> &str;

    fn get_state_enum(&self) -> StateTypes;

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error>;

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error>;

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
            history: vec![]
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(PendingReview {}))
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            match s.approve() {
                Ok(t) => {

                    self.state = Some(t);
                    println!("Approved !");
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
        let msg = self.state.as_ref().unwrap().print_state();
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
        Box::new(PendingReview {})
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

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error>{
        println!("Draft objects can't be rejected !");
        Ok(self)
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("It is still reviewing");
        self
    }

    fn print_state(&self) -> &str {
        "Pending Reviewing ..."
    }

    fn get_state_enum(&self) -> StateTypes {
        StateTypes::PendingReview
    }

    fn approve(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        Ok(Box::new(Published {}))
    }

    fn reject(self: Box<Self>) -> Result<Box<dyn State>, Error> {
        Ok(Box::new(Draft {}))
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("It is already reviewed !");
        self
    }

    fn print_state(&self) -> &str {
        "Published !"
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
}