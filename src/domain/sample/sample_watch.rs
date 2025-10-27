use crate::domain::watch::Watch;

pub struct SampleOutput {
    pub id: String,
    pub time: String,
    pub name: String
}

#[derive(PartialEq)]
pub struct SampleWatch {

}

impl Watch for SampleWatch {
    fn label(&self) -> &str {
        "Sample Watch"
    }
    
    fn title(&self) -> &str {
        "Sample watch"
    }
    
    fn watch(&self) {
        println!("Perform watch")
    }
}