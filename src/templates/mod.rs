pub mod blog;
pub mod home;

pub trait WithSiteContext {
    fn site_name(&self) -> &'static str {
        "LogforT"
    }
}
