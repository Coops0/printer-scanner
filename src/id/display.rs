use crate::id::devices::{BuildingPageType, DellController, NetworkDevice, Printer};
use std::fmt::{Display, Formatter};

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

impl Display for DellController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DellController::Eight => f.write_str("8"),
            DellController::Nine => f.write_str("9"),
        }
    }
}

impl Display for BuildingPageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildingPageType::Login => f.write_str("Login"),
            BuildingPageType::Controller => f.write_str("Controller"),
        }
    }
}
