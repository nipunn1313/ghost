use nom::{bytes::complete::take, IResult, InputIter, InputTake};

/// Grab 4 characters from input
fn take4<I>(input: I) -> IResult<I, I>
where
    I: InputIter + InputTake,
{
    take(4usize)(input)
}

/// Grab 4 characters and parse as a length
fn takelength<I>(input: I) -> IResult<I, u64>
where
    I: InputIter + InputTake,
{
    unimplemented!()
}

/*
fn hexadecimal_value(input: Bytes) -> IResult<Bytes, u64> {
    map_res(
        take(4),
        |out: &str| u64::from_str_radix(out, 16),
    )(input)
}
*/

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};
    use nom::Err;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_take4() {
        assert_eq!(take4("001e#other stuff"), Ok(("#other stuff", "001e")));
        assert_eq!(take4("001e"), Ok(("", "001e")));
        assert_eq!(take4("001"), Err(Err::Error(Error::new("001", ErrorKind::Eof))));
    }

    /*
    #[test]
    fn test_hex_value() {
        hexadecimal_value(b"001e")
    }
    */
}
