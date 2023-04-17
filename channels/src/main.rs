#![allow(unused)]

use channels::channels::passing_rx::{self, Sample, SampleData};
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use std::{
    fmt::Error,
    string::ParseError,
    thread::{self, Thread},
};

fn main() -> Result<(), ParseError> {
    let mut sample_data = SampleData::new();
    let (ex_tx, ex_rx) = mpsc::channel();
    let mut handle = vec![];

    for x in 0..10 {
        let ex_tx_cl = ex_tx.clone();
        handle.push(thread::spawn(move || {
            ex_tx_cl.send(x);
        }));
    }

    for h in handle {
        sample_data.data.push(passing_rx::set_by_receiver(&ex_rx));
    }

    dbg!(sample_data.get_len());

    println!("Data :: {:?}", serde_json::to_string(&sample_data).unwrap());

    Ok(())
}
