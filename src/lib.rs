mod list;
pub use api_response::error_code::ErrType;
pub use list::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            if cfg!(feature = "chinese_description") {
                "用户端错误 ErrType(1000)"
            } else {
                "User Error ErrType(1000)"
            },
            ET_USER.to_string()
        );
    }
}
