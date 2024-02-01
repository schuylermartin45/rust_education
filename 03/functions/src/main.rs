/// Demo function
/// This comment generates `markdown` when `cargo` produces an HTML doc
/// * `x` - Value to print
/// * `lbl` -  Unit to attach to the value
fn lbl_measurement(x: i32, lbl: char) {
    println!("Measurement is: {x}{lbl}");
}

fn main() {
    let y = {
        let x = 3;
        x + if true { 4 } else { 1 }
    };
    lbl_measurement(y, 'F');

    for n in 0..5 {
        lbl_measurement(n, 'L');
    }
}