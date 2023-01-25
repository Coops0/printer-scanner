use crate::util::IpWrapper;

pub mod devices;
mod display;
mod elements;

pub trait PageElement {
    fn page_element(&self, ip: &IpWrapper) -> Option<String>;
}
