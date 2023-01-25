use enum_iterator::{all, Sequence};

use crate::id::PageElement;
use crate::util::IpWrapper;

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

#[derive(Debug, Clone)]
pub enum BuildingPageType {
    Login,
    Controller,
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

impl NetworkDevice {
    #[allow(clippy::needless_pass_by_value)]
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
                NetworkDevice::Fortinet,
            ],
            all::<Printer>()
                .collect::<Vec<_>>()
                .into_iter()
                .map(NetworkDevice::HpPrinter)
                .collect::<Vec<_>>(),
        ]
        .concat()
    }
}

impl Printer {
    #[allow(dead_code)]
    pub fn is_new(&self) -> bool {
        matches!(
            self,
            Printer::LaserJetMfpM528
                | Printer::ColorLaserJetFlowMfpM681
                | Printer::LaserJetMfpM635
                | Printer::OfficeJetPro8720
        )
    }

    pub fn is_unknown(&self) -> bool {
        matches!(
            self,
            Printer::UnknownOfficeJet
                | Printer::UnknownLaserJet
                | Printer::UnknownJavascriptPrinter
        )
    }
}
