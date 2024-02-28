use crate::{engine::Engine, mainui};
use anyhow::{bail, Result};
use dialog::FileDialogType::BrowseFile;
use fltk::{
    app::{self, App, Receiver, Scheme},
    dialog,
    prelude::{BrowserExt, WidgetExt},
};
use fltk_desk::ui::misc::Theme;
#[derive(Copy, Clone)]
pub enum Message {
    Quit,
    LoadBinary,
    SegListPick,
    MenuEditCut,
}

pub struct CxApp {
    app: App,
    channel: Receiver<Message>,
    ui: mainui::UI,
    engine: Engine,
    symhid: bool,
}
pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

impl CxApp {
    pub fn new(theme: &'static Theme) -> Result<Self> {
        let (s, r) = app::channel::<Message>();
        let app = App::default().with_scheme(Scheme::Gtk);
        let mut ui = mainui::UI::new(&s, theme);
        ui.show();
        Ok(Self {
            app,
            channel: r,
            ui,
            engine: Engine::new()?,
            symhid: false,
        })
    }
    pub fn run(&mut self) {
        while self.app.wait() {
            if let Some(msg) = self.channel.recv() {
                let r = self.dispatch_message(msg);
                match r {
                    Ok(true) => break,
                    Ok(false) => (),
                    Err(e) => {
                        println!("Error: {}", e);
                        dialog::alert(
                            center().0 - 200,
                            center().1 - 100,
                            &format!("An issue occured while loading the file: {e}"),
                        );
                    }
                }
            }
        }
    }
    fn dispatch_message(&mut self, msg: Message) -> Result<bool> {
        match msg {
            Message::Quit => return Ok(true),
            Message::LoadBinary => {
                let mut nfc = dialog::NativeFileChooser::new(BrowseFile);
                nfc.show();

                let filename = nfc.filename();
                if !filename.to_string_lossy().to_string().is_empty() {
                    self.engine.load_code(nfc.filename().as_path())?;
                    self.ui.load_seg_list(&self.engine.seg_list);
                }
            }
            Message::SegListPick => {
                let idx = self.ui.seglist.selected_row();
                let row = self.ui.seglist.get_row(idx.unwrap());
                if let Some(r) = row {
                    let segname = r.cells[0].clone();
                    let segs = &self.engine.seg_list;
                    if let Some(seg) = segs.iter().find(|s| s.name.as_str() == segname) {
                        self.engine.load_cx_data(seg.id as _, 0, 0xffff)?;
                        self.ui.csource.clear();
                        for span in self.engine.cview.iter() {
                            let filename = self
                                .engine
                                .lookup_file_by_id(span.file_id as _)
                                .map_or("", |si| &si.short_name);
                            let text = self
                                .engine
                                .find_source_line_by_line_no(span.file_id, span.line_no)?
                                .map_or_else(|| String::new(), |si| si.line);
                            self.ui.csource.append(&format!(
                                "{}:{}\t{}\t{}\n",
                                filename, span.line_no, text, span.absaddr
                            ));
                        }
                    } else {
                        bail!("unknown segment {}", segname);
                    }
                }

                self.ui.seglist.redraw();
            }
            Message::MenuEditCut => {
                let mut rect = self.ui.seglist.get_rect();
                rect.h = 166;
                rect.y = 34;
                self.ui.seglist.set_rect(rect);

                let mut rect = self.ui.symlist.get_rect();
                rect.h = 500;
                rect.y = 34 + 166;
                self.ui.symlist.set_rect(rect);

                self.ui.seglist.redraw();

                self.ui.symlist.redraw();
                //    self.main_win.tile.redraw();
            } //self.main_win.tile
        }
        Ok(false)
    }
}
