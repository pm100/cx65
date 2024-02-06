// For a simpler boilerplate-less table, check the fltk-table crate



use std::{cell::RefCell, rc::Rc, sync::Arc};

use fltk::{
    app::Sender, enums::Event, prelude::*, table::{TableRow, TableRowSelectMode}, *
};

use crate::cxapp::Message;
#[derive(Debug, Clone,  PartialEq, Eq, Hash)]
pub struct ColumnDefinition{
    pub name: String,
    pub width: i32,
   // pub resizable: bool,
  //  pub  sortable: bool,
 
  //  pub  tag: Option<String>
}
#[derive(Debug, Clone,  PartialEq, Eq, Hash)]
pub struct Row{
  pub  cells: Vec<String>,
  pub   tag: Option<String>
}
pub struct ListBox{
    table: TableRow,
    rows: Arc<RefCell<Vec<Row>>>,
    cols:Rc<Vec<ColumnDefinition>>
}

pub struct Rect{
 pub    x: i32,
  pub  y: i32,
   pub w: i32,
     pub h: i32
}

impl ListBox{
    pub fn new(size:Rect, cols:&[ColumnDefinition], label: &str) -> Self{
        let mut table = TableRow::new(size.x, size.y, size.w, size.h, "").with_type(TableRowSelectMode::Single);
        table.set_rows(0);
        table.set_row_header(false);
        table.set_row_resize(false);
        table.set_cols(cols.len() as i32);
        table.set_col_header(true);
        for (i, col) in cols.iter().enumerate(){
            table.set_col_width(i as i32, col.width);
          //  table.set_col_resize(i as i32, col.resizable);
            //table.set_col_sort(i as i32, col.sortable, col.sort_order.unwrap_or(enums::Sort::Ascending));
        }
        let mut  s =  ListBox{
            table,
            rows: Arc::default(),
            cols: Rc::new(cols.to_vec())
        };
       // s.table.make_resizable(true);
       
        s.table.set_col_resize(true);
        s.table.draw_cell({
            let cols = s.cols.clone();
         
            let rows = s.rows.clone();
            move  |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(enums::Font::Courier, 14),
            table::TableContext::ColHeader => {
                Self::draw_header(&cols,col, x, y, w, h)
            }
            
            table::TableContext::Cell => {
             //   print!("xxx");
                let cell = rows.borrow().get(row as usize).unwrap().cells.get(col as usize).unwrap().to_string();
               // let sel = t.row_selected(row);
               let (rt,ct,rb,cb) = t.get_selection();
             //  println!("{:?} {:?} {:?} {:?}", rt,ct,rb,cb);
               let sel = rt == row;
                    Self::draw_data(
                &cell.as_str(),
                x,
                y,
                w,
                h,
                sel,
            )}
            , // Data in cells
            _ => (),
        }});
        s.table.end();
s
    }
    pub fn redraw(&mut self){
        self.table.redraw();
    }
    pub fn add_row(&mut self, row: Row){
        self.rows.borrow_mut().push(row);
        self.table.set_rows(self.rows.borrow_mut().len() as i32);
    }
    pub fn clear(&mut self){
        self.rows.borrow_mut().clear();
    }
    pub fn emit(&mut self, channel: Sender<Message>, msg: Message){
        self.table.emit(channel, msg);
    }
    pub fn selected_row(&mut self) -> Option<i32>{
        let (rt,ct,rb,cb) = self.table.get_selection();
        println!("{:?} {:?} {:?} {:?}", rt,ct,rb,cb);
        println!("{:?}", self.table.row_selected(rt));
        Some(rt)
    }
    pub fn get_row(&self, row: i32) -> Option<Row>{
     let b = self.rows.borrow();
     let r = b[row as usize].clone();
        Some(r)

    }
    fn draw_header(cols:&Rc<Vec<ColumnDefinition>>, col:i32, x: i32, y: i32, w: i32, h: i32) {
        draw::push_clip(x, y, w, h);
        draw::draw_box(
            enums::FrameType::ThinUpBox,
            x,
            y,
            w,
            h,
            enums::Color::FrameDefault,
        );
        let txt = cols[col as usize].name.as_str();
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Helvetica, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
        draw::pop_clip();
    }
    fn draw_data(txt:&str,  x: i32, y: i32, w: i32, h: i32, selected: bool) {
        draw::push_clip(x, y, w, h);
        if selected {
            draw::set_draw_color(enums::Color::Blue);
        } else {
            draw::set_draw_color(enums::Color::White);
        }
        draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Courier, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Left);
        //  draw::draw_rect(x, y, w, h);
        draw::pop_clip();
    }
}
/* 
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut table = table::TableRow::default()
        .with_size(800 - 10, 600 - 10)
        .with_type(TableRowSelectMode::Single)
        .center_of(&wind);

    table.set_rows(30);
    table.set_row_header(false);
    table.set_row_resize(false);
    table.set_cols(26);
    table.set_col_header(true);
    table.set_col_width_all(80);
    table.set_col_resize(true);
    //table.with_type(TableRowSelectMode::Single);
    table.end();
    //table.set_callback(win_cb);
    table.handle(win_cb2);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Called when the table is drawn then when it's redrawn due to events
    table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
        table::TableContext::ColHeader => {
            draw_header(&format!("{}", (col + 65) as u8 as char), x, y, w, h)
        } // Column titles
        //table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h), // Row titles

        table::TableContext::Cell => {
            let sel = t.row_selected(row);
                draw_data(
            &format!("{}", row + col),
            x,
            y,
            w,
            h,
            sel,
        )}
        , // Data in cells
        _ => (),
    });

    app.run().unwrap();
}
fn win_cb(_w: &mut TableRow) {
    println!("{:?}", app::event());
    println!("{} {}", app::event_x(), app::event_y());
    println!("{} ", app::event_button());
}
fn win_cb2(_w: &mut TableRow, ev: Event) -> bool {
    println!("{:?}", ev);
    println!("{} {}", app::event_x(), app::event_y());
    println!("{} ", app::event_button());
    true
}
fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox,
        x,
        y,
        w,
        h,
        enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
    } else {
        draw::set_draw_color(enums::Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    draw::set_font(enums::Font::Courier, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Left);
    //  draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
*/