#[derive(Debug, PartialEq, Eq)]
pub enum SeriesNameError {
    NotExisting,
}

pub struct Series(String);

impl Series {
    pub fn new(series: String) -> Series {
        Series(series)
    }

    pub fn name(&self) -> Result<&str, SeriesNameError> {
        match self.0.as_str() {
            "401" => Ok("ICE 1"),
            "402" => Ok("ICE 2"),
            "411" | "415" => Ok("ICE T"),
            "403" | "406" | "407" => Ok("ICE 3"),
            "412" => Ok("ICE 4"),
            _ => Err(SeriesNameError::NotExisting),
        }
    }
}

impl From<Series> for String {
    fn from(series: Series) -> String {
        series.0
    }
}

impl From<&Series> for String {
    fn from(series: &Series) -> String {
        series.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            Series::new(String::from("401")).name(), Ok("ICE 1"));
    }
}
