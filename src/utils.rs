pub fn get_input(day: u8) -> String {
    unsafe {
        std::string::String::from_utf8_unchecked(
            std::fs::read(format!("resources/day{:02}input.txt", day)).unwrap(),
        )
    }
}
