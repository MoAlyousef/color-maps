use color_maps::*;

fn main() {
    let html_black = html::HTML_MAP.get("Black").unwrap();
    assert_eq!(*html_black, (0, 0, 0));
    
    let x_black = x::X_MAP.get("black").unwrap();
    assert_eq!(*x_black, (0, 0, 0));
}