use crate::id::devices::{BuildingPageType, DellController, NetworkDevice, Printer};
use crate::id::PageElement;
use crate::util::IpWrapper;

impl PageElement for NetworkDevice {
    fn page_element(&self, ip: &IpWrapper) -> Option<String> {
        let res = match self {
            NetworkDevice::CiscoRouter => "<script>window.onload=function(){ url ='/webui';window.location.href=url;}</script>",
            NetworkDevice::FileMaker => "FileMaker Database Server Website",
            NetworkDevice::VirataEmWeb => "Access Denied. Your IP Address cannot access this device",
            NetworkDevice::MitsubishiAC => "MITSUBISHI Air Conditioning Control System",
            NetworkDevice::MiVoice => "MiVoice Office Communications Platform",
            _ => "",
        };

        if let NetworkDevice::HpPrinter(p) = self {
            return p.page_element(ip);
        }

        if let NetworkDevice::IntegrateDellRemoveAccessController(d) = self {
            return d.page_element(ip);
        }

        if let NetworkDevice::BuildingOperations(b) = self {
            return b.page_element(ip);
        }

        if matches!(self, NetworkDevice::Fortinet) {
            return Some(format!("<a href=\"https://{ip}/ng\">here</a>.</p"));
        }

        if res.is_empty() {
            None
        } else {
            Some(res.to_string())
        }
    }
}

impl PageElement for BuildingPageType {
    fn page_element(&self, _ip: &IpWrapper) -> Option<String> {
        let s = match self {
            BuildingPageType::Login => "<button type=\"submit\" id=\"login\"></button></label>",
            BuildingPageType::Controller => "h5.02c.518 0 .918-.187 1.255-.56.12-.147.28",
        };

        Some(s.to_string())
    }
}

impl PageElement for DellController {
    fn page_element(&self, ip: &IpWrapper) -> Option<String> {
        let s = match self {
            DellController::Eight => format!("<a href=\"https://{}/start.html\">here</a>", ip.0),
            DellController::Nine => format!("<a href=\"https://{}/restgui/start.html\">here</a>", ip.0),
        };

        Some(s)
    }
}

impl PageElement for Printer {
    fn page_element(&self, _ip: &IpWrapper) -> Option<String> {
        if self.is_unknown() {
            None
        } else {
            Some(self.to_string())
        }
    }
}
