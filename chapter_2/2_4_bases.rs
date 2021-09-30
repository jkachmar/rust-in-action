fn main() {
    let three = 0b11; // Binary literal.
    let thirty = 0o36; // Octal literal.
    let three_hundred = 0x12C; // Hex literal.

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
