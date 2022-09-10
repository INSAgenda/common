use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Department {
    Ai2,
    Ai3,
    Ai4,
    Asi5,
    Cgc3,
    Cgc4,
    Cp5,
    EuromedSic,
    Ep3,
    Ep4,
    Ep5,
    Gc5,
    Gcu3,
    Gcu4,
    Gm3,
    Gm4,
    Gm5,
    Gpgr3,
    Gpgr4,
    Iti3,
    Iti4,
    Mic1,
    Mic2,
    Meca3,
    Meca4,
    Meca5,
    Mri5,
    MsEsd,
    PerfE3,
    PerfE4,
    PerfE5,
    PerfIi3,
    PerfIi4,
    PerfIi5,
    PerfIsp3,
    PerfIsp4,
    PerfIsp5,
    PerfNi3,
    PerfNi4,
    PerfNi5,
    Stpi1,
    Stpi2,
}

impl FromStr for Department{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AI2" => Ok(Department::Ai2),
            "AI3" => Ok(Department::Ai3),
            "AI4" => Ok(Department::Ai4),
            "ASI5" => Ok(Department::Asi5),
            "CGC3" => Ok(Department::Cgc3),
            "CGC4" => Ok(Department::Cgc4),
            "CP5" => Ok(Department::Cp5),
            "EUROMED-SIC" => Ok(Department::EuromedSic),
            "EP3" => Ok(Department::Ep3),
            "EP4" => Ok(Department::Ep4),
            "EP5" => Ok(Department::Ep5),
            "GC5" => Ok(Department::Gc5),
            "GCU3" => Ok(Department::Gcu3),
            "GCU4" => Ok(Department::Gcu4),
            "GM3" => Ok(Department::Gm3),
            "GM4" => Ok(Department::Gm4),
            "GM5" => Ok(Department::Gm5),
            "GPGR3" => Ok(Department::Gpgr3),
            "GPGR4" => Ok(Department::Gpgr4),
            "ITI3" => Ok(Department::Iti3),
            "ITI4" => Ok(Department::Iti4),
            "M-IC1" => Ok(Department::Mic1),
            "M-IC2" => Ok(Department::Mic2),
            "MECA3" => Ok(Department::Meca3),
            "MECA4" => Ok(Department::Meca4),
            "MECA5" => Ok(Department::Meca5),
            "MRI5" => Ok(Department::Mri5),
            "MS-ESD" => Ok(Department::MsEsd),
            "PERF-E3" => Ok(Department::PerfE3),
            "PERF-E4" => Ok(Department::PerfE4),
            "PERF-E5" => Ok(Department::PerfE5),
            "PERF-II3" => Ok(Department::PerfIi3),
            "PERF-II4" => Ok(Department::PerfIi4),
            "PERF-II5" => Ok(Department::PerfIi5),
            "PERF-ISP3" => Ok(Department::PerfIsp3),
            "PERF-ISP4" => Ok(Department::PerfIsp4),
            "PERF-ISP5" => Ok(Department::PerfIsp5),
            "PERF-NI3" => Ok(Department::PerfNi3),
            "PERF-NI4" => Ok(Department::PerfNi4),
            "PERF-NI5" => Ok(Department::PerfNi5),
            "STPI1" => Ok(Department::Stpi1),
            "STPI2" => Ok(Department::Stpi2),
            _ => Err(format!("Impossible to parse: {} into a Department enum.", s))
        }
    }
}

impl From<&Department> for &'static str {
    fn from(obj: &Department) -> Self {
        match obj {
            Department::Ai2 => "AI2",
            Department::Ai3 => "AI3",
            Department::Ai4 => "AI4",
            Department::Asi5 => "ASI5",
            Department::Cgc3 => "CGC3",
            Department::Cgc4 => "CGC4",
            Department::Cp5 => "CP5",
            Department::EuromedSic => "EUROMED-SIC",
            Department::Ep3 => "EP3",
            Department::Ep4 => "EP4",
            Department::Ep5 => "EP5",
            Department::Gc5 => "GC5",
            Department::Gcu3 => "GCU3",
            Department::Gcu4 => "GCU4",
            Department::Gm3 => "GM3",
            Department::Gm4 => "GM4",
            Department::Gm5 => "GM5",
            Department::Gpgr3 => "GPGR3",
            Department::Gpgr4 => "GPGR4",
            Department::Iti3 => "ITI3",
            Department::Iti4 => "ITI4",
            Department::Mic1 => "M-IC1",
            Department::Mic2 => "M-IC2",
            Department::Meca3 => "MECA3",
            Department::Meca4 => "MECA4",
            Department::Meca5 => "MECA5",
            Department::Mri5 => "MRI5",
            Department::MsEsd => "MS-ESD",
            Department::PerfE3 => "PERF-E3",
            Department::PerfE4 => "PERF-E4",
            Department::PerfE5 => "PERF-E5",
            Department::PerfIi3 => "PERF-II3",
            Department::PerfIi4 => "PERF-II4",
            Department::PerfIi5 => "PERF-II5",
            Department::PerfIsp3 => "PERF-ISP3",
            Department::PerfIsp4 => "PERF-ISP4",
            Department::PerfIsp5 => "PERF-ISP5",
            Department::PerfNi3 => "PERF-NI3",
            Department::PerfNi4 => "PERF-NI4",
            Department::PerfNi5 => "PERF-NI5",
            Department::Stpi1 => "STPI1",
            Department::Stpi2 => "STPI2",
        }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&'static str>::from(self))
    }
}

impl Department {
    pub fn list() -> &'static [Department] {
        &[Department::Stpi1, Department::Stpi2]
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}
