use nom::IResult;
use nom::error::ParseError;
use nom::bits::streaming::take as bits_take;
use nom::combinator::{map,verify};
use nom::sequence::tuple;
use crate::FLACStream;
use crate::parser::metadata::stream_info::parse_stream_info;
use crate::metadata::stream_info::StreamInfo;
use crate::parser::metadata::{parse_metadata_block_header, BlockHeader};

pub mod metadata;

fn parse_magic<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), (), E>
    where E: ParseError<(&'a [u8], usize)>
{
    let bytes_4 = tuple((
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
    ));
    let checker = verify(bytes_4, |bs: &(u8, u8, u8, u8)|
        bs.0 == 0x66 && bs.1 == 0x4C && bs.2 == 0x61 && bs.3 == 0x43
    );
    map(checker, |_| ())(input)
}

pub fn parse_flac_stream<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), FLACStream, E>
    where E: ParseError<(&'a [u8], usize)>
{
    let magic = parse_magic;
    let first_block_header = parse_metadata_block_header;
    let stream_info = parse_stream_info;

    let parser = tuple((
        magic,
        first_block_header,
        stream_info,
    ));
    map(parser, |i: ((), BlockHeader, StreamInfo)| FLACStream::new(i.2))(input)
}

#[cfg(test)]
mod test {
    use crate::parser::{parse_magic, parse_flac_stream};
    use crate::metadata::stream_info::StreamInfo;
    use crate::FLACStream;

    #[test]
    fn test_parse_magic() {
        let test_case = [b'f', b'L', b'a', b'C'];
        let res = parse_magic::<()>((&test_case, 0));
        assert!(res.is_ok());
    }

    #[test]
    fn test_parse_flac_stream() {
        let stream = [b'f', b'L', b'a', b'C', 0x80, 0x00, 0x00, 0x22, 0x00, 0x10, 0x24, 0x00, 0x00, 0x00, 0x21,0x04, 0x20, 0x00, 0x0B, 0xB8, 0x03, 0xF0, 0x00, 0x83, 0xD6, 0x00, 0x0B, 0x60, 0xDB, 0x9F, 0x4B, 0xA2, 0xED, 0xB2, 0x90, 0x29, 0x59, 0xAC, 0xF0, 0x1F, 0x8F, 0x32];
        let res = parse_flac_stream::<()>((&stream, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &stream[42..]);
        assert_eq!(res.0.1, 0);
        let expected_stream_info = StreamInfo::new(0x0010, 0x2400, 0x000021, 0x042000, 48000, 2, 32, 8640000);
        let expected_flac_stream = FLACStream::new(expected_stream_info);
        assert_eq!(res.1, expected_flac_stream);
    }
}
