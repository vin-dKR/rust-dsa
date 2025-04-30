// &str -> (Shared Reference to a String Slice)
pub fn reverse_string(str: &str) -> String {
    if str.is_empty() {
        return String::new();
    }

    // You're not mutating the &str. Here's what happens:
    // split_at(n) returns two new string slices:
    let (first, rest) = str.split_at(1);

    format!("{}{}", reverse_string(rest), first)
}
