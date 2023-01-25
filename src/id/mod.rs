use crate::util::IpWrapper;

pub mod devices;
mod elements;
mod display;

pub trait PageElement {
    fn page_element(&self, ip: &IpWrapper) -> Option<String>;
}