use core::fmt::{Display, Formatter};
use enum_iterator::{all, Sequence};
use crate::scanner::IpWrapper;

#[derive(Clone)]
pub enum NetworkDevice {
    // version
    IntegrateDellRemoveAccessController(DellController),
    CiscoRouter,
    HpPrinter(Printer),
    FileMaker,
    VirataEmWeb,
    MitsubishiAC,
    BuildingOperations(BuildingPageType),
    MiVoice,
    Fortinet,
    Unidentified,
}

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

impl Display for NetworkDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            NetworkDevice::IntegrateDellRemoveAccessController(v) => f.write_fmt(format_args!("Integrate Dell Remove Access Controller {v}")),
            NetworkDevice::CiscoRouter => f.write_str("Cisco Router"),
            NetworkDevice::HpPrinter(p) => f.write_fmt(format_args!("HP Printer {p}")),
            NetworkDevice::FileMaker => f.write_str("FileMaker Database Server Website"),
            NetworkDevice::VirataEmWeb => f.write_str("Viarta EmWeb"),
            NetworkDevice::MitsubishiAC => f.write_str("Mitsubishi Air Conditioning"),
            NetworkDevice::BuildingOperations(d) => f.write_fmt(format_args!("Building Operations {d}")),
            NetworkDevice::MiVoice => f.write_str("MiVoice"),
            NetworkDevice::Fortinet => f.write_str("Fortinet"),
            NetworkDevice::Unidentified => f.write_str("Unidentified"),
        }
    }
}

impl NetworkDevice {
    pub fn from_response(ip: &IpWrapper, text: String) -> Self {
        for t in NetworkDevice::all() {
            if let Some(e) = t.page_element(ip) {
                if text.contains(&e) {
                    return t;
                }
            }
        }

        if text.contains("HP LaserJet") {
            return Self::HpPrinter(Printer::UnknownLaserJet);
        }

        if text.contains("HP OfficeJet") {
            return Self::HpPrinter(Printer::UnknownOfficeJet);
        }

        if text.contains("/framework/Unified.css") {
            return Self::HpPrinter(Printer::UnknownJavascriptPrinter);
        }

        Self::Unidentified
    }
}

#[derive(Debug, Clone, Sequence)]
pub enum Printer {
    LaserJetMfpM528,
    LaserJet600M602,
    OfficeJetPro8702,
    ColorLaserJetMfpM577,
    ColorLaserJetM750,
    LaserJetM402dne,
    LaserJetM402dn,
    LaserJetM605,
    LaserJetProMfpM521dn,
    ColorLaserJetCp5520Series,
    LaserJetM506,
    LaserJetM402n,
    LaserJetMfpM527,
    LaserJetMfpM227fdw,
    LaserJet500MfpM525,
    ColorLaserJetFlowMfpM681,
    LaserJetM203dw,
    LaserJetMfpM426fdw,
    LaserJetMfpM635,
    OfficeJetPro8720,

    UnknownJavascriptPrinter,
    UnknownLaserJet,
    UnknownOfficeJet,
}

#[derive(Debug, Clone, Sequence)]
pub enum DellController {
    Eight,
    Nine,
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

impl Display for DellController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DellController::Eight => f.write_str("8"),
            DellController::Nine => f.write_str("9"),
        }
    }
}

impl Printer {
    #[allow(dead_code)]
    pub fn is_new(&self) -> bool {
        matches!(self, Printer::LaserJetMfpM528 | Printer::ColorLaserJetFlowMfpM681 | Printer::LaserJetMfpM635 | Printer::OfficeJetPro8720)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Printer::UnknownOfficeJet | Printer::UnknownLaserJet | Printer::UnknownJavascriptPrinter)
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

trait PageElement {
    fn page_element(&self, ip: &IpWrapper) -> Option<String>;
}

impl NetworkDevice {
    fn all() -> Vec<Self> {
        vec![
            vec![
                NetworkDevice::IntegrateDellRemoveAccessController(DellController::Eight),
                NetworkDevice::IntegrateDellRemoveAccessController(DellController::Nine),
            ],
            vec![
                NetworkDevice::BuildingOperations(BuildingPageType::Controller),
                NetworkDevice::BuildingOperations(BuildingPageType::Login),
            ],
            vec![
                NetworkDevice::CiscoRouter,
                NetworkDevice::FileMaker,
                NetworkDevice::MitsubishiAC,
                NetworkDevice::VirataEmWeb,
                NetworkDevice::MiVoice,
                NetworkDevice::Fortinet
            ],
            all::<Printer>()
                .collect::<Vec<_>>()
                .into_iter()
                .map(NetworkDevice::HpPrinter)
                .collect::<Vec<_>>(),
        ].concat()
    }
}


impl Display for Printer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Printer::LaserJetMfpM528 => "LaserJet MFP M528",
            Printer::LaserJet600M602 => "LaserJet 600 M602",
            Printer::OfficeJetPro8702 => "OfficeJet Pro 8702",
            Printer::ColorLaserJetMfpM577 => "Color LaserJet MFP M577",
            Printer::ColorLaserJetM750 => "Color LaserJet M750",
            Printer::LaserJetM402dne => "LaserJet M402dne",
            Printer::LaserJetM402dn => "LaserJet M402dn",
            Printer::LaserJetM605 => "LaserJet M605",
            Printer::LaserJetProMfpM521dn => "LaserJet Pro MFP M521dn",
            Printer::ColorLaserJetCp5520Series => "Color LaserJet CP5520 Series",
            Printer::LaserJetM506 => "LaserJet M506",
            Printer::LaserJetM402n => "LaserJet M402n",
            Printer::LaserJetMfpM527 => "LaserJet MFP M527",
            Printer::LaserJetMfpM227fdw => "LaserJet MFP M227fdw",
            Printer::LaserJet500MfpM525 => "LaserJet 500 MFP M525",
            Printer::ColorLaserJetFlowMfpM681 => "Color LaserJet FlowMFP M681",
            Printer::LaserJetM203dw => "LaserJet M203dw",
            Printer::LaserJetMfpM426fdw => "LaserJet MFP M426fdw",
            Printer::LaserJetMfpM635 => "LaserJet MFP M635",
            Printer::OfficeJetPro8720 => "OfficeJet Pro 8720",

            Printer::UnknownJavascriptPrinter => "Unknown Javascript Printer",
            Printer::UnknownLaserJet => "Unknown LaserJet",
            Printer::UnknownOfficeJet => "Unknown OfficeJet",
        };

        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub enum BuildingPageType {
    Login,
    Controller,
}

impl Display for BuildingPageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingPageType::Login => f.write_str("Login"),
            BuildingPageType::Controller => f.write_str("Controller"),
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