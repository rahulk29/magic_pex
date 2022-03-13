use magic_vlsi::MagicInstance;

use crate::{
    error::Result,
    verification::pex::{Pex, PexInput, PexOpts, PexOutput},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct MagicPex {}

impl Pex<PexOpts> for MagicPex {
    fn pex(input: PexInput<PexOpts>) -> Result<PexOutput> {
        std::fs::create_dir_all(&input.work_dir)?;
        let mut m = MagicInstance::builder()
            .cwd(&input.work_dir)
            .tech(&input.tech)
            .port(portpicker::pick_unused_port().expect("no free ports"))
            .build()
            .unwrap();
        m.drc_off()?;
        m.set_snap(magic_vlsi::SnapMode::Internal)?;

        let cell_path = input
            .layout
            .to_owned()
            .into_os_string()
            .into_string()
            .unwrap();
        m.load(&cell_path)?;

        m.exec_one("ext2spice format ngspice")?;
        m.exec_one("ext2spice rthresh 0")?;
        m.exec_one("ext2spice cthresh 0")?;
        m.exec_one("ext2spice extresist on")?;
        m.exec_one("ext2spice resistor tee on")?;
        m.exec_one("ext2spice hierarchy on")?;
        m.exec_one("ext2spice scale off")?;
        m.exec_one("ext2spice global off")?;
        m.exec_one("ext2spice subcircuit top auto")?;
        m.exec_one("ext2spice renumber off")?;

        m.exec_one("ext do all")?;
        m.exec_one("ext do local")?;
        m.exec_one("ext do capacitance")?;
        m.exec_one("ext do resistance")?;
        m.exec_one("ext do coupling")?;
        m.exec_one("ext do length")?;

        m.exec_one("ext all")?;
        m.exec_one("ext2spice")?;

        let path = input.work_dir.join(format!("{}.spice", input.layout_cell));

        Ok(PexOutput { netlist: path })
    }
}
