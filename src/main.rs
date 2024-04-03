use color_print::cprintln;
fn main() {
    println!("FIZZBUZZ Generator: If the number is divisible by 3, FIZZ is echoed, if it is divisible by 5, BUZZ is echoed, if it is divisible by, if it is divisible by, if it is divisible by, if it is div by both 3 and 5 FIZZBUZZ is echoed");
    println!("===================================================================================================================");
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            cprintln!("<bold><bg:bright-white><green>FIZZBUZZ</></></>");
        } else if i % 3 == 0 && i % 5 != 0 {
            cprintln!("<bold><bg:bright-white><red>FIZZ</></></>")
        } else if i % 5 == 0 && i % 3 != 0 {
            cprintln!("<bold><bg:bright-white><blue>BUZZ</></></>");
        } else {
            println!("{}", i);
        }
    }
}
