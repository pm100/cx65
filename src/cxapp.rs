use crate::{engine::Engine, mainui};
use anyhow::{bail, Result};
use dialog::FileDialogType::BrowseFile;
use fltk::{
    app::{self, App, Receiver, Scheme},
    dialog,
    prelude::BrowserExt,
};
#[derive(Copy, Clone)]
pub enum Message {
    Quit,
    LoadBinary,
    SegList,
}

pub struct CxApp {
    app: App,
    channel: Receiver<Message>,
    main_win: mainui::UI,
    engine: Engine,
}
pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

impl CxApp {
    pub fn new() -> Result<Self> {
        let (s, r) = app::channel::<Message>();
        let app = App::default().with_scheme(Scheme::Gtk);
        let mut ui = mainui::UI::new(&s);
        ui.show();
        Ok(Self {
            app,
            channel: r,
            main_win: ui,
            engine: Engine::new()?,
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
                    self.engine.load_dbg_file(nfc.filename().as_path())?;
                    self.main_win.load_seg_list(&self.engine.seg_list);
                }
            }
            Message::SegList => {
                let idx = self.main_win.seglist.selected_row();
                let row = self.main_win.seglist.get_row(idx.unwrap());
                if let Some(r) = row {
                    let segname = r.cells[0].clone();
                    let segs = &self.engine.seg_list;
                    if let Some(seg) = segs.iter().find(|s| s.name.as_str() == segname) {
                        self.main_win.set_segment(&seg);
                    } else {
                        bail!("unknown segment {}", segname);
                    }
                }
                // for i in self.main_win.seglist.selected_items().iter() {
                //     println!("Selected: {}", i);
                //     let segname: &str = unsafe { self.main_win.seglist.data(*i).unwrap() };
                //     let segs = &self.engine.seg_list;
                //     if let Some(seg) = segs.iter().find(|s| s.name.as_str() == segname) {
                //         self.main_win.set_segment(&seg);
                //     } else {
                //         bail!("unknown segment {}", segname);
                //     }
                // }
                self.main_win.seglist.redraw();
            }
        }
        Ok(false)
    }
}
