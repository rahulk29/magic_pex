use std::path::PathBuf;

use crate::verification::pex::{Pex, PexInput, PexOpts};

use super::MagicPex;

#[test]
fn test_pex_sky130() -> Result<(), Box<dyn std::error::Error>> {
    let work_dir: PathBuf = "/tmp/sram22/tests/pex/nand2".into();
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let layout =
        base.join("src/verification/plugins/magic_pex/tests/data/clean/nand2_dec_auto.mag");
    let _output = MagicPex::pex(PexInput {
        layout,
        layout_cell: "nand2_dec_auto".to_string(),
        work_dir,
        tech: "sky130A".to_string(),
        opts: PexOpts {},
    })?;
    Ok(())
}
