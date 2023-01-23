use core::fmt::{Display, Formatter};
use enum_iterator::{all, Sequence};

#[derive(Clone)]
pub enum NetworkDevice {
    // version
    IntegrateDellRemoveAccessController(u8),
    CiscoRouter,
    HpPrinter(Printer),
    FileMaker,
    Unidentified,
}

impl NetworkDevice {
    pub fn from_response(text: String) -> Self {
        for t in NetworkDevice::all() {
            if let Some(e) = t.page_element() {
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
    LaserJetM605,
    LaserJetProMfpM521dn,
    ColorLaserJetCp5520Series,
    LaserJetM506,
    LaserJetM402n,
    LaserJetMfpM527,
    LaserJetMfpM227fdw,
    LaserJet500MfpM525,

    UnknownLaserJet,
    UnknownOfficeJet,
}

impl Printer {
    #[allow(dead_code)]
    pub fn is_new(&self) -> bool {
        matches!(self, Printer::LaserJetMfpM528)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Printer::UnknownOfficeJet | Printer::UnknownLaserJet)
    }
}

impl PageElement for Printer {
    fn page_element(&self) -> Option<String> {
        if self.is_unknown() {
            None
        } else {
            Some(self.to_string())
        }
    }
}

trait PageElement {
    fn page_element(&self) -> Option<String>;
}

impl NetworkDevice {
    fn all() -> Vec<Self> {
        vec![
            vec![
                NetworkDevice::IntegrateDellRemoveAccessController(8),
                NetworkDevice::IntegrateDellRemoveAccessController(9),
            ],
            vec![
                NetworkDevice::CiscoRouter,
                NetworkDevice::FileMaker,
            ],
            all::<Printer>()
                .collect::<Vec<_>>()
                .into_iter()
                .map(NetworkDevice::HpPrinter)
                .collect::<Vec<_>>(),
        ].concat()
    }
}

impl Display for NetworkDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            NetworkDevice::IntegrateDellRemoveAccessController(v) => f.write_fmt(format_args!("Integrate Dell Remove Access Controller {v}")),
            NetworkDevice::CiscoRouter => f.write_str("Cisco Router"),
            NetworkDevice::HpPrinter(p) => {
                f.write_str("HP Printer ")?;
                p.fmt(f)
            }
            NetworkDevice::FileMaker => f.write_str("FileMaker Database Server Website"),
            NetworkDevice::Unidentified => f.write_str("Unidentified"),
        }
    }
}

impl Display for Printer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Printer::LaserJetMfpM528 => "LaserJet MFP M528",
            Printer::LaserJet600M602 => "LaserJet 600 M602",
            Printer::OfficeJetPro8702 => "OfficeJet Pro 8702",
            Printer::ColorLaserJetMfpM577 => "Color LaserJet MFP M577",
            Printer::ColorLaserJetM750 => "Color LaserJet M570",
            Printer::LaserJetM402dne => "LaserJet M402dne",
            Printer::LaserJetM605 => "LaserJet M605",
            Printer::LaserJetProMfpM521dn => "LaserJet Pro MFP M521dn",
            Printer::ColorLaserJetCp5520Series => "Color LaserJet CP5520 Series",
            Printer::LaserJetM506 => "LaserJet M506",
            Printer::LaserJetM402n => "LaserJet M402n",
            Printer::LaserJetMfpM527 => "LaserJet MFP M527",
            Printer::LaserJetMfpM227fdw => "LaserJet MFP M227fdw",
            Printer::LaserJet500MfpM525 => "LaserJet 500 MFP M525",
            Printer::UnknownLaserJet => "Unknown LaserJet",
            Printer::UnknownOfficeJet => "Unknown OfficeJet",
        };

        f.write_str(s)
    }
}

impl PageElement for NetworkDevice {
    fn page_element(&self) -> Option<String> {
        let res = match self {
            NetworkDevice::IntegrateDellRemoveAccessController(9) => r#"/restgui/start.html""#,
            NetworkDevice::IntegrateDellRemoveAccessController(8) => r#"/start.html">"#,
            NetworkDevice::CiscoRouter => r#"<script>window.onload=function(){ url ='/webui';window.location.href=url;}</script>"#,
            NetworkDevice::FileMaker => "FileMaker Database Server Website",
            _ => "",
        };

        if let NetworkDevice::HpPrinter(p) = self {
            return p.page_element();
        }

        if res.is_empty() {
            None
        } else {
            Some(res.to_string())
        }
    }
}