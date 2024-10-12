pub trait Truncate {
    fn truncate(&self, max_width: usize) -> String;
}

impl Truncate for String {
    fn truncate(&self, max_width: usize) -> String {
        if self.len() > max_width {
            format!("{}...", &self[..max_width - 3])
        } else {
            self.to_string()
        }
    }
}
