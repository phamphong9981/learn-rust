pub fn get_home_env() -> Option<&'static str> {
    option_env!("HOME")
}

pub fn get_test_env() -> Option<&'static str> {
    option_env!("TEST")
}
