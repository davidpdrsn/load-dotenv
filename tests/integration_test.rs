use load_dotenv::{try_load_dotenv, load_dotenv};

load_dotenv!();

#[test]
fn test() {
    assert_eq!("value", env!("KEY"));
}
