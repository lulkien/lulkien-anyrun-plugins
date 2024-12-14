use crate::notify::Notify;

pub struct PowerEntry {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub icon: String,
    pub keywords: String,
    pub notify: Option<Notify>,
    pub id: u64,
}
