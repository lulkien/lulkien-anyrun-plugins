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

impl PowerEntry {
    pub fn formatted_title(&self, hl_indices: &[usize], hl_color: &str) -> String {
        let mut result = String::new();
        for (i, c) in self.name.chars().enumerate() {
            if hl_indices.contains(&i) {
                result.push_str(&format!(
                    "<span weight=\"bold\"color=\"{}\">{}</span>",
                    hl_color, c
                ));
            } else {
                result.push(c);
            }
        }

        result
    }
}
