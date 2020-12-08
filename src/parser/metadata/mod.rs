use nom::IResult;
use nom::error::ParseError;
use nom::bits::streaming::take as bits_take;
use nom::combinator::map;
use nom::sequence::tuple;

pub mod stream_info;

#[derive(PartialEq, Debug)]
enum BlockType {
    StreamInfo,
    Padding,
    Application,
    SeekTable,
    VorbisComment,
    CueSheet,
    Picture,
    Reserved,
    Invalid,
}

#[derive(PartialEq, Debug)]
pub struct BlockHeader {
    is_last_block: bool,
    block_type: BlockType,
    length: u32,
}

fn parse_last_metadata_block_flag<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), bool, E>
    where E: ParseError<(&'a [u8], usize)>
{
    let bits_flag = bits_take(1usize);
    map(bits_flag, |b: u8| b != 0)(input)
}

fn parse_block_type<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), BlockType, E>
    where E: ParseError<(&'a [u8], usize)>
{
    let bits_7 = bits_take(7usize);
    map(bits_7, |bs: u8| match bs {
        0 => BlockType::StreamInfo,
        1 => BlockType::Padding,
        2 => BlockType::Application,
        3 => BlockType::SeekTable,
        4 => BlockType::VorbisComment,
        5 => BlockType::CueSheet,
        6 => BlockType::Picture,
        127 => BlockType::Invalid,
        _ => BlockType::Reserved,
    })(input)
}

fn parse_block_length<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), u32, E>
    where E: ParseError<(&'a [u8], usize)>
{
    let bits_24 = tuple((
        bits_take(8usize),
        bits_take(8usize),
        bits_take(8usize),
    ));
    map(bits_24, |bs: (u8, u8, u8)| bs.0 as u32 * 0x10000 + bs.1 as u32 * 0x100 + bs.2 as u32)(input)
}

pub fn parse_metadata_block_header<'a, E>(input: (&'a [u8], usize)) -> IResult<(&'a [u8], usize), BlockHeader, E>
    where E: ParseError<(&'a [u8], usize)>
{
    let last_metadata_block_flag = parse_last_metadata_block_flag;
    let block_type = parse_block_type;
    let block_length = parse_block_length;

    let parser = tuple((
        last_metadata_block_flag,
        block_type,
        block_length,
    ));
    map(parser, |i: (bool, BlockType, u32)| BlockHeader {
        is_last_block: i.0,
        block_type: i.1,
        length: i.2,
    })(input)
}

#[cfg(test)]
mod test {
    use crate::parser::metadata::{parse_last_metadata_block_flag, parse_block_type, BlockType, parse_block_length, parse_metadata_block_header, BlockHeader};

    #[test]
    fn test_parse_last_metadata_block_flag() {
        let test_case = [0x80];
        let res = parse_last_metadata_block_flag::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[0..]);
        assert_eq!(res.0.1, 1);
        assert_eq!(res.1, true);
    }

    #[test]
    fn test_parse_block_type() {
        let test_case = [0x08];
        let res = parse_block_type::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[0..]);
        assert_eq!(res.0.1, 7);
        assert_eq!(res.1, BlockType::VorbisComment);
    }

    #[test]
    fn test_parse_block_length() {
        let test_case = [0x00, 0x00, 0x22];
        let res = parse_block_length::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[3..]);
        assert_eq!(res.0.1, 0);
        assert_eq!(res.1, 34);
    }

    #[test]
    fn test_parse_metadata_block_header() {
        let test_case = [0x04, 0x00, 0x00, 0x22];
        let res = parse_metadata_block_header::<()>((&test_case, 0));
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.0.0, &test_case[4..]);
        assert_eq!(res.0.1, 0);
        let expected = BlockHeader {
            is_last_block: false,
            block_type: BlockType::VorbisComment,
            length: 34,
        };
        assert_eq!(res.1, expected);
    }
}
