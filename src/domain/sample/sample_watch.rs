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
    fn label(&self) -> String {
        "Sample Watch".to_string()
    }
    
    fn title(&self) -> String {
        "Sample watch".to_string()
    }
    
    fn watch(&self) {
        println!("Perform watch")
    }
}