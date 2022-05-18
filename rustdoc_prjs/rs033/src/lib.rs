#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

#[allow(unused)]
pub struct Screen1<T: Draw> {
    pub components: Vec<T>
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "width: {}, height: {}, label: {}",
            self.width,
            self.height,
            self.label,
        )
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl<T> Screen1<T>
where
    T: Draw,
{
    #[allow(unused)]
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

impl AveragedCollection {
    pub fn new(l: Vec<i32>, a: f64) -> AveragedCollection {
        AveragedCollection {
            list: l,
            average: a,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    #[allow(unused)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
struct Draft {}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

pub struct Post1 {
    content: String,
}

impl Post1 {
    pub fn new() -> DraftPost1 {
        DraftPost1 {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost1 {
    content: String,
}

impl DraftPost1 {
    pub fn add_text(&mut self, s: &str) {
        self.content.push_str(s);
    }
    pub fn request_review(self) -> PendingReviewPost1 {
        PendingReviewPost1 {
            content: self.content
        }
    }
}

pub struct PendingReviewPost1 {
    content: String,
}

impl PendingReviewPost1 {
    pub fn approve(self) -> Post1 {
        Post1 {
            content: self.content
        }
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}