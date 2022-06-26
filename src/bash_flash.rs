pub mod init {
    pub fn welcome() {
        println!("Welcome to Bash Flash!")
    }
}

pub mod io {
    // impl later
}

pub mod start {
    use super::init::welcome;

    pub fn run() {
        welcome();
    }
}