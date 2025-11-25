use cxx::CxxString;

mod ffi;
pub mod graph_builder;
pub mod graph_info;
pub mod graph_reader;

fn cxx_string_to_string(value: &CxxString) -> String {
    value
        .to_str()
        .map(|s| s.to_owned())
        .unwrap_or_else(|_| String::from_utf8_lossy(value.as_bytes()).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cxx::let_cxx_string;

    #[test]
    fn test_cxx_string_to_string_basic() {
        let_cxx_string!(s = "hello");
        assert_eq!(cxx_string_to_string(&s), "hello");

        let_cxx_string!(empty = "");
        assert_eq!(cxx_string_to_string(&empty), "");
    }
}
