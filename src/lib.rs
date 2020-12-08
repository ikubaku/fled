#![no_std]

use crate::metadata::stream_info::StreamInfo;

pub mod metadata;
pub mod parser;

#[derive(PartialEq, Debug)]
pub struct FLACStream {
    stream_info: StreamInfo,
}

impl FLACStream {
    pub fn new(stream_info: StreamInfo) -> FLACStream {
        FLACStream {
            stream_info,
        }
    }
}
