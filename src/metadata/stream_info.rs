#[derive(Debug, PartialEq)]
pub struct StreamInfo {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_rate: u32,
    n_channels: u8,
    bits_per_sample: u8,
    n_total_samples: u64,
    //md5: u128,
}

impl StreamInfo {
    pub fn new(min_block_size: u16, max_block_size: u16, min_frame_size: u32, max_frame_size: u32, sample_rate: u32, n_channels: u8, bits_per_sample: u8, n_total_samples: u64) -> StreamInfo {
        StreamInfo{
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            n_channels,
            bits_per_sample,
            n_total_samples,
        }
    }
}
