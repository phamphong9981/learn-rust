pub fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        // In điều kiện
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1)) // In kết quả đệ quy
    }
}
