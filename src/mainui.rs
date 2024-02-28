use dbgdata::debugdb::{Segment, SourceInfo};
use fltk::{
    app, browser,
    draw::Rect,
    enums::{self, Color, Font, FrameType, Shortcut},
    group, menu,
    prelude::{BrowserExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window,
};
use fltk_desk::ui::{
    control::Control,
    listbox::{ColumnDefinition, ListBox, Row},
    misc::Theme,
    splitter::Splitter,
    textbox::TextBox,
};

use crate::cxapp::Message;

pub struct UI {
    main_win: window::Window,
    menu: MainMenu,
    // pub tile: group::Tile,
    theme: &'static Theme,
    pub seglist: ListBox,
    pub symlist: ListBox,
    pub csource: TextBox, //<Message>,
    //pub browser2: browser::HoldBrowser,
    send_channel: app::Sender<Message>,
}

impl UI {
    pub fn new(channel: &app::Sender<Message>, theme: &'static Theme) -> UI {
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("cx65 - Code Explorer for the cc65 toolchain");
        let menu = MainMenu::new(channel, theme);
        let mut vsplitter = Splitter::new(
            Rect {
                x: 0,
                y: 34,
                w: 800,
                h: 600 - 34,
            },
            theme,
            true,
        );
        let mut hsplitter = Splitter::new(
            Rect {
                x: 0,
                y: 34,
                w: 800,
                h: 600 - 34,
            },
            theme,
            false,
        );
        // tile.set_color(enums::Color::White);

        let cols = vec![
            ColumnDefinition {
                name: "Name".to_string(),
                width: 100,
            },
            ColumnDefinition {
                name: "Start".to_string(),
                width: 40,
            },
            ColumnDefinition {
                name: "Size".to_string(),
                width: 40,
            },
        ];
        let mut seglist = ListBox::new(
            Rect {
                x: 0,
                y: 34,
                w: 400,
                h: (600 - 34) / 2,
            },
            &cols,
            theme,
        );
        let sym_cols = vec![
            ColumnDefinition {
                name: "Name".to_string(),
                width: 100,
            },
            ColumnDefinition {
                name: "Value".to_string(),
                width: 40,
            },
            ColumnDefinition {
                name: "Type".to_string(),
                width: 40,
            },
        ];
        let mut symlist = ListBox::new(
            Rect {
                x: 0,
                y: 34 + (600 - 34) / 2,
                w: 400,
                h: (600 - 34) / 2,
            },
            &sym_cols,
            theme,
        );
        // seglist.emit(*channel, Message::SegListPick);
        hsplitter.add(seglist.clone());
        hsplitter.add(symlist.clone());
        vsplitter.add(hsplitter);
        //  let mut c = browser::HoldBrowser::new(400, 35, 400, 600 - 35, "1");
        let csource = TextBox::new(
            Rect {
                x: 400,
                y: 35,
                w: 400,
                h: 600 - 35,
            },
            theme,
        );
        vsplitter.add(csource.clone());
        main_win.add(&vsplitter.fl_widget());
        //  main_win.resizable(&tile);
        main_win.end();
        UI {
            main_win,
            menu,
            // tile,
            seglist,
            symlist,
            csource,
            //  browser2: c,
            send_channel: channel.clone(),
            theme,
        }
    }
    pub fn load_seg_list(&mut self, seg_list: &Vec<Segment>) {
        // let widths = &[100, 50, 50];
        // self.seglist.set_column_widths(widths);
        // self.seglist.set_label_font(Font::Courier);
        // self.seglist.set_column_char('\t');
        // for seg in seg_list.iter() {
        //     self.seglist.add_with_data(
        //         &format!("@t{}\t{}\t{}", seg.name, seg.start, seg.size),
        //         seg.name.clone(),
        //     );
        // }
        self.seglist.clear();
        for seg in seg_list.iter() {
            self.seglist.add_row(Row {
                cells: vec![
                    seg.name.clone(),
                    seg.start.to_string(),
                    seg.size.to_string(),
                ],
                tag: None,
            });
        }
    }
    pub fn show(&mut self) {
        self.main_win.show();
    }
    pub fn update_cx(&mut self, data: &Vec<SourceInfo>) {
        self.csource.clear();
        for span in data.iter() {
            self.csource.append(span.line.as_str());
        }
    }
    pub fn set_segment(&mut self, seg: &Segment) {
        self.csource.clear();

        for chunk in seg.modules.iter() {
            // self.browser2.add(&format!(
            //     "@t{}\t{}\t{}",
            //     chunk.module_name, chunk.offset, chunk.size
            // ));
        }

        //  self.browser2.add(&format!("Segment: {}", segname));
    }
}

pub struct MainMenu {
    menu: menu::SysMenuBar,
}
fn menu_cb(m: &mut impl MenuExt) {
    if let Ok(mpath) = m.item_pathname(None) {
        println!("Menu '{}'", mpath);
    }
}
impl MainMenu {
    pub fn new(channel: &app::Sender<Message>, theme: &Theme) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 34);
        //    menu.set_color(theme.bg);
        menu.set_color(theme.popbg);
        menu.set_text_color(theme.fg);
        menu.set_frame(FrameType::FlatBox);
        //      menu.set_text_color(theme.fg);
        //        menu.set_frame(FrameType::FlatBox);
        app::set_menu_linespacing(15);
        menu.set_selection_color(theme.hl);
        menu.add_emit(
            "  &File/Load Binary..\t",
            Shortcut::Ctrl | 'l',
            menu::MenuFlag::Normal,
            *channel,
            Message::LoadBinary,
        );

        menu.add(
            "  &File/Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "  &File/Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "  &File/Save as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "  &File/Print...\t",
            Shortcut::Ctrl | 'p',
            menu::MenuFlag::MenuDivider,
            menu_cb,
        );

        menu.add_emit(
            "  &File/Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *channel,
            Message::Quit,
        );

        menu.add_emit(
            "&Edit/Cut\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            *channel,
            Message::MenuEditCut,
        );

        menu.add(
            "&Edit/Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&Edit/Paste\t",
            Shortcut::Ctrl | 'v',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&Help/About\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            menu_cb,
        );

        Self { menu }
    }
}
