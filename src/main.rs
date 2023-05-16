#![allow(unused_variables)]

fn main() {
    let sum = 3 +4;
    let modulo = 40 % 26;
    println!("sum: {sum}, modulo: {modulo}");

    // Exponencial
    let quadrado = i64::pow(8, 2);
    println!("quadrado: {quadrado}");

    // Bitwise
    let and = 0b0011 & 0b0101;
    let or = 0b0011 | 0b0101;
    let xor = 0b0011 ^ 0b0101;
    let not = !0b0011;
    let shift_left = 0b0011 << 1;
    let shift_right = 0b0011 >> 1;
    println!("and: {and}, or: {or}, xor: {xor}, not: {not}, shift_left: {shift_left}, shift_right: {shift_right}");
}
