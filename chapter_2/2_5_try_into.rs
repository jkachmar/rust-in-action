use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // let b_ = b.try_into().unwrap();

    // if a < b_ {
    //     println!("Ten is less than one hundred.");
    // }

    // if let Ok(b_conv) = b.try_into() {
    //     if a < b_conv {
    //         ()
    //     }
    //     println!("Ten is less than one hundred.");
    // }

    match b.try_into() {
        Ok(b_conv) if a < b_conv => {
            println!("Ten is less than one hundred.");
        }
        _ => (),
    };
}
