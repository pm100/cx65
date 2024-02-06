use dbgdata::debugdb::Segment;
use fltk::{
    app, browser,
    enums::{self, Color, Font, FrameType, Shortcut},
    group, menu,
    prelude::{BrowserExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window,
};

use crate::{
    cxapp::Message,
    listbox::{ColumnDefinition, ListBox, Rect, Row},
};

pub struct UI {
    main_win: window::Window,
    menu: MainMenu,
    tile: group::Tile,
    pub seglist: ListBox, //browser::HoldBrowser,
    browser2: browser::HoldBrowser,
    send_channel: app::Sender<Message>,
}

impl UI {
    pub fn new(channel: &app::Sender<Message>) -> UI {
        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("cx65 - Code Explorer for the cc65 toolchain");
        let menu = MainMenu::new(channel);
        main_win.set_color(Color::White);
        let mut tile = group::Tile::new(0, 35, 800, 600 - 35, None);
        tile.set_color(enums::Color::White);
        // let mut seglist = browser::HoldBrowser::new(0, 35, 400, 600 - 35, "0");
        let cols = vec![
            ColumnDefinition {
                name: "Name".to_string(),
                width: 100,
            },
            ColumnDefinition {
                name: "Start".to_string(),
                width: 30,
            },
            ColumnDefinition {
                name: "Size".to_string(),
                width: 40,
            },
        ];
        let mut seglist = ListBox::new(
            Rect {
                x: 0,
                y: 35,
                w: 400,
                h: 600 - 35,
            },
            &cols,
            "seglist",
        );
        seglist.emit(*channel, Message::SegList);

        let mut c = browser::HoldBrowser::new(400, 35, 400, 600 - 35, "1");
        main_win.make_resizable(true);
        // tile.resizable(&main_win);
        main_win.resizable(&tile);
        main_win.end();
        UI {
            main_win,
            menu,
            tile,
            seglist,
            browser2: c,
            send_channel: channel.clone(),
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
    pub fn set_segment(&mut self, seg: &Segment) {
        self.browser2.clear();

        for chunk in seg.modules.iter() {
            self.browser2.add(&format!(
                "@t{}\t{}\t{}",
                chunk.module_name, chunk.offset, chunk.size
            ));
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
    pub fn new(channel: &app::Sender<Message>) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 35);

        menu.set_frame(FrameType::FlatBox);
        menu.add_emit(
            "&File/Load Binary..\t",
            Shortcut::Ctrl | 'l',
            menu::MenuFlag::Normal,
            *channel,
            Message::LoadBinary,
        );

        menu.add(
            "&File/Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Save as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Print...\t",
            Shortcut::Ctrl | 'p',
            menu::MenuFlag::MenuDivider,
            menu_cb,
        );

        menu.add_emit(
            "&File/Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *channel,
            Message::Quit,
        );

        menu.add(
            "&Edit/Cut\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            menu_cb,
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
