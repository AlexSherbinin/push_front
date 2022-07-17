pub trait PushFront {
    fn push_front(&mut self, ch: char);
    fn push_front_str(&mut self, string: &str);
}

impl PushFront for String {
    /// Appends the given char to the front of this String
    fn push_front(&mut self, ch: char) {
        self.insert(0, ch);
    }
    /// Appends a given string slice onto the front of this String
    fn push_front_str(&mut self, string: &str) {
        self.insert_str(0, string);
    }
}