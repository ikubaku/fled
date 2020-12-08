use nom::IResult;
use nom::error::ParseError;
use nom::bits::streaming::take as bits_take;
use nom::combinator::map;
use nom::sequence::tuple;
use crate::metadata::stream_info::StreamInfo;


fn parse_block_size<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u16, E>
where E: ParseError<(&'a [u8], usize)>
{
    let be_16 = tuple((
        bits_take(8usize),
        bits_take(8usize),
    ));
    map(be_16, |bs: (u8, u8)| bs.0 as u16 * 0x100 + bs.1 as u16)(input)
}

fn parse_frame_size<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u32, E>
where E: ParseError<(&'a [u8], usize)>
{
    let be_24 = tuple((
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
    ));
    map(be_24, |bs: (u8, u8, u8)| bs.0 as u32 * 0x10000 + bs.1 as u32 * 0x100 + bs.2 as u32)(input)
}

fn parse_sample_rate<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u32, E>
where E: ParseError<(&'a [u8], usize)>
{
    let be_20 = tuple((
        bits_take(4usize),
        bits_take(8usize),
        bits_take(8usize),
    ));
    map(be_20, |bs: (u8, u8, u8)| bs.0 as u32 * 0x10000 + bs.1 as u32 * 0x100 + bs.2 as u32)(input)
}

fn parse_n_channels<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u8, E>
where E: ParseError<(&'a [u8], usize)>
{
    let bits_3 = bits_take(3usize);
    map(bits_3, |b: u8| b + 1)(input)
}

fn parse_bits_per_sample<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u8, E>
where E: ParseError<(&'a [u8], usize)>
{
    let bits_5 = bits_take(5usize);
    map(bits_5, |b: u8| b + 1)(input)
}

fn parse_n_total_samples<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u64, E>
where E: ParseError<(&'a [u8], usize)>
{
    let be_36 = tuple((
        bits_take(4usize),
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
    ));
    map(be_36, |bs: (u8, u8, u8, u8, u8)| bs.0 as u64 * 0x100000000 + bs.1 as u64 * 0x1000000 + bs.2 as u64 * 0x10000 + bs.3 as u64 * 0x100 + bs.4 as u64)(input)
}

pub fn parse_stream_info<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), StreamInfo, E>
where E: ParseError<(&'a [u8], usize)>
{
    let min_block_size = parse_block_size;
    let max_block_size = parse_block_size;
    let min_frame_size = parse_frame_size;
    let max_frame_size = parse_frame_size;
    let sample_rate = parse_sample_rate;
    let n_channels = parse_n_channels;
    let bits_per_sample = parse_bits_per_sample;
    let n_total_samples = parse_n_total_samples;
    let md5_hash = tuple((
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
        bits_take::<_, u8, _, _>(8usize),
    ));    // Dummy parser

    let parser = tuple((
        min_block_size,
        max_block_size,
        min_frame_size,
        max_frame_size,
        sample_rate,
        n_channels,
        bits_per_sample,
        n_total_samples,
        md5_hash,
    ));

    map(parser, |i: (u16, u16, u32, u32, u32, u8, u8, u64, _)| StreamInfo::new(i.0, i.1, i.2, i.3, i.4, i.5, i.6, i.7))(input)
}

#[cfg(test)]
mod test {
    use crate::parser::metadata::stream_info::{parse_block_size, parse_frame_size, parse_sample_rate, parse_n_channels, parse_bits_per_sample, parse_n_total_samples, parse_stream_info};
    use crate::metadata::stream_info::StreamInfo;

    #[test]
    fn test_parse_block_size() {
        let test_case = [0x01, 0x10];
        let res = parse_block_size::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[2..]);
        assert_eq!(res.0.1, 0);
        assert_eq!(res.1, 0x0110);
    }

    #[test]
    fn test_parse_frame_size() {
        let test_case = [0x01, 0x10, 0x10];
        let res = parse_frame_size::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[3..]);
        assert_eq!(res.0.1, 0);
        assert_eq!(res.1, 0x011010);
    }

    #[test]
    fn test_parse_sample_rate() {
        let test_case = [0x0A, 0xC4, 0x40];
        let res = parse_sample_rate::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[2..]);
        assert_eq!(res.0.1, 4);
        assert_eq!(res.1, 44100);
    }

    #[test]
    fn test_parse_n_channels() {
        let test_case = [0x20];
        let res = parse_n_channels::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[0..]);
        assert_eq!(res.0.1, 3);
        assert_eq!(res.1, 2);
    }

    #[test]
    fn test_parse_bits_per_sample() {
        let test_case = [0xB8];
        let res = parse_bits_per_sample::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[0..]);
        assert_eq!(res.0.1, 5);
        assert_eq!(res.1, 24);
    }

    #[test]
    fn test_parse_n_total_samples() {
        let test_case = [0x00, 0x07, 0x91, 0xFD, 0x00];
        let res = parse_n_total_samples::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[4..]);
        assert_eq!(res.0.1, 4);
        assert_eq!(res.1, 44100 * 60 * 3);
    }

    #[test]
    fn test_parse_stream_info() {
        // See docs/test/test_parse_stream_info.case.txt for details.
        let test_case = [0x00, 0x10, 0x24, 0x00, 0x00, 0x00, 0x21,0x04, 0x20, 0x00, 0x0B, 0xB8, 0x03, 0xF0, 0x00, 0x83, 0xD6, 0x00, 0x0B, 0x60, 0xDB, 0x9F, 0x4B, 0xA2, 0xED, 0xB2, 0x90, 0x29, 0x59, 0xAC, 0xF0, 0x1F, 0x8F, 0x32];
        let res = parse_stream_info::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[34..]);
        assert_eq!(res.0.1, 0);
        let expected = StreamInfo::new(0x0010, 0x2400, 0x000021, 0x042000, 48000, 2, 32, 8640000);
        assert_eq!(res.1, expected);
    }
}
