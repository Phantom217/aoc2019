use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    Ok((String::new(), String::new()))
}
