use anyhow::Result;
use dbgdata::debugdb::{DebugData, Segment};
use std::{fs::File, io::BufReader, path::Path};

pub struct Engine {
    dbgdata: DebugData,
    pub seg_list: Vec<Segment>,
}
impl Engine {
    pub fn new() -> Result<Self> {
        let dbgdb = DebugData::new("cx65.db")?;
        Ok(Self {
            dbgdata: dbgdb,
            seg_list: Vec::new(),
        })
    }
    pub fn load_dbg_file(&mut self, file: &Path) -> Result<()> {
        let fd = File::open(file)?;
        let mut reader = BufReader::new(fd);
        self.dbgdata.clear()?;
        self.dbgdata.parse(&mut reader)?;
        self.dbgdata.load_seg_list(&mut self.seg_list)?;
        Ok(())
    }
}
