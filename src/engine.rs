use anyhow::Result;
use dbgdata::debugdb::{DebugData, Segment, SourceFile, SourceInfo};
use dbgdata::loader;
use std::{fs::File, io::BufReader, path::Path};
use util::say;

pub struct Engine {
    dbgdata: DebugData,
    pub seg_list: Vec<Segment>,
    pub cview: Vec<SourceInfo>,
    ram: [u8; 0x10000],
    dbg_suffix: String,
}
impl Engine {
    pub fn new() -> Result<Self> {
        let dbgdb = DebugData::new("cx65.db")?;
        Ok(Self {
            dbgdata: dbgdb,
            seg_list: Vec::new(),
            cview: Vec::new(),
            ram: [0; 0x10000],
            dbg_suffix: ".dbg".to_string(),
        })
    }
    pub fn load_code(&mut self, file: &Path) -> Result<()> {
        let (sp65_addr, run, cpu, size) =
            loader::load_c64_code(file, &mut |a, b| self.ram[a as usize] = b)?;
        let mut pb = file.canonicalize()?;
        pb.pop(); // drop the file
        self.dbgdata.set_load_path(&pb)?;
        if let Some(prefix) = file.file_stem() {
            let prefix = prefix.to_str().unwrap();
            let mut path = file.to_path_buf();
            path.pop();
            path.push(format!("{}{}", prefix, self.dbg_suffix));

            if path.exists() {
                say!("Loading debug info from {:?}", path);
                self.load_dbg_file(&path)?;
            }
        }
        Ok(())
    }
    pub fn load_dbg_file(&mut self, file: &Path) -> Result<()> {
        let fd = File::open(file)?;
        let mut reader = BufReader::new(fd);
        self.dbgdata.clear()?;
        self.dbgdata.parse(&mut reader)?;
        self.dbgdata.load_seg_list(&mut self.seg_list)?;
        self.load_all_cfiles()?;
        Ok(())
    }
    pub fn load_cx_data(&mut self, seg: i64, from: u16, to: u16) -> Result<()> {
        let data = self.dbgdata.load_cx_view(seg, from, to)?;
        self.cview = data;
        Ok(())
    }
    pub fn lookup_file_by_id(&self, id: i64) -> Option<&SourceFile> {
        self.dbgdata.lookup_file_by_id(id)
    }

    pub fn load_all_cfiles(&mut self) -> Result<()> {
        self.dbgdata.load_all_cfiles()
    }

    pub fn find_source_line_by_line_no(
        &self,
        file: i64,
        line_no: i64,
    ) -> Result<Option<SourceInfo>> {
        self.dbgdata.find_source_line_by_line_no(file, line_no)
    }
}
