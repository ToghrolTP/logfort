use chrono::{Datelike, Local};

pub mod blog;
pub mod home;

pub trait WithSiteContext {
    fn site_name(&self) -> &'static str {
        "LogforT"
    }

    fn current_year(&self) -> i32 {
        Local::now().year()
    }
}
