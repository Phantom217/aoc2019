use crate::computer::{Computer, Rom};
use crate::error::Error;

pub fn run<R>(reader: R) -> Result<(String, String), Error>
where
    R: std::io::BufRead,
{
    Ok((String::new(), String::new()))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_day11() {}
}
