use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct F64(f64);

impl std::convert::TryFrom<f64> for F64 {
    type Error = Error;

    fn try_from(f: f64) -> Result<Self, Self::Error> {
        if f.is_nan() {
            bail!("Cannot convert {} into F64.", f);
        }

        Ok(F64(f))
    }
}
impl std::ops::Deref for F64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Eq for F64 {}

impl std::hash::Hash for F64 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.to_bits().hash(state);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Vec2<T>(T, T);

impl<T> Vec2<T> {
    pub(crate) const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T> Vec2<T> where T: Copy {
    pub(crate) fn x(&self) -> T {
        self.0
    }

    pub(crate) fn y(&self) -> T {
        self.1
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::error::Error;

    pub(crate) fn test_full_problem<F>(day: usize, run_func: F, expected1: &str, expected2: &str)
    where
        F: Fn(std::io::BufReader<std::fs::File>) -> Result<(String, String), Error>,
    {
        let path = format!("input/day{:02}.txt", day);
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let (actual1, actual2) = run_func(reader).unwrap();
        assert_eq!(&actual1, expected1);
        assert_eq!(&actual2, expected2);
    }
}
