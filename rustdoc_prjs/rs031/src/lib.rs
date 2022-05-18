pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracer<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracer<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracer<T> {
        LimitTracer {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 98% of your quota!");
        } else if percentage_of_max >=0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));

            // let mut one_borrow = self.sent_message.borrow_mut();
            // let mut two_borrow = self.sent_message.borrow_mut(); // already borrowed: BorrowMutError

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenger::new();
        let mut limit_tracer = LimitTracer::new(&mock_message, 100);
        limit_tracer.set_value(80);
        assert_eq!(mock_message.sent_message.borrow().len(), 1);
    }
}