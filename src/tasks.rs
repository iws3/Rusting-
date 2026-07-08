pub mod priority;
use priority::Priority;

pub struct Task {
    pub title:String,
    pub priority:Priority,
    pub description:Option<String>,
}

impl Task {
    pub fn new(title:&str, priority:Priority, description:Option<String>) -> Task {
        Task {
            title:title.to_string(),
            priority,
            description,
        }
    }

    pub fn display(&self) {
        println!("Title: {}", self.title);
        println!("Priority: {:?}", self.priority);
        match &self.description {
            Some(desc) => println!("Description: {}", desc),
            None => println!("Description: None"),
        }
    }   
}