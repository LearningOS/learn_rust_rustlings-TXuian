// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

fn main() {
    let mut data = "Rust is great!".to_string();

    // data is still needed afterward
    // use reference
    println!("{}", get_char(&data));

    // data should be mutable
    // cannot use reference as param
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
