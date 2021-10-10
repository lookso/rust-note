pub mod bmod {
    pub fn b() -> i32 {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        number
    }
}
