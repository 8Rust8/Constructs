use std::sync::mpsc::Receiver;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleData {
    pub data: Vec<Sample>,
}

impl SampleData {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn get_len(&self) -> usize {
        self.data.len()
    }
}

impl Sample {
    pub fn new(number: Option<i32>) -> Self {
        Self { number }
    }
}

pub fn set_by_receiver(rx: &Receiver<i32>) -> Sample {
    match rx.try_recv() {
        Ok(number) => Sample::new(Some(number)),
        Err(_) => Sample::new(None),
    }
}
