pub mod lifetimes {
    use colored::Colorize;
    pub fn print_str(instr: &str) {
        println!("Input is :: {} ", instr.red().bold());
    }
}
