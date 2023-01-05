use crossterm::style::Stylize;

pub struct Logger;

impl Logger {
    /// Log a valid output
    ///
    /// # Arguments
    ///
    /// * `msg` - &str
    pub fn info(msg: &str) {
        println!("✔️ {}", msg.green());
    }

    /// Log a warning message
    ///
    /// # Arguments
    ///
    /// * `msg` - &str
    pub fn warn<T: ToString>(msg: T) {
        println!("⚠️  {}", msg.to_string().yellow());
    }


    /// Show a notice message
    ///
    /// # Arguments
    ///
    /// * `msg` - T
    pub fn notice<T: ToString>(msg: T) {
        println!("{}", msg.to_string().trim().blue());
    }

    /// Show an error message
    ///
    /// # Arguments
    ///
    /// * `msg` - &str
    /// * `highlight` - &str
    pub fn error<T: ToString>(msg: &str, highlight: T) {
        println!("❌ {} {}", msg.red(), highlight.to_string().bold());
    }
}
