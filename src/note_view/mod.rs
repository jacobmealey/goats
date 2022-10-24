pub mod imp;

use glib::Object;
use gtk::prelude::*;
use std::sync::Arc;
use gtk::WrapMode;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;
use gtk::glib;

use std::sync::Mutex;

glib::wrapper! {
    pub struct NoteViewObject(ObjectSubclass<imp::NoteViewObject>)
    @extends gtk::TextView, gtk::Widget, gtk::gio::SimpleActionGroup,
    @implements gtk::Accessible, gtk::Buildable,  
    gtk::ConstraintTarget, gtk::Orientable;
}

impl NoteViewObject {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `NoteView`.")
    }

    // this function is going to be paired w/ a deserialize function
    // The goal is to insert tags from the tag table inline in a mark
    // up style format - so for bold text it will be:
    //      <bold> some text </bold>
    //  and Italics will be :
    //      <italic> some text </italic> 
    //
    // Right now this is just scaffoling - but I think it could use an 
    // "accumulator" string which characters are pushed to and if that 
    // iterator is also a tag start or end push <bold> or <italic>
    // 
    // ideally we can also use this for formatting bulleted and numbered
    // lists. 
    pub fn serialize(&self) {
        let (start, end) = self.buffer().bounds();
        let mut iter = start;
        let mut open_tag = gtk::TextTag::new(Some("filler"));
        while iter != end {
            for tag in iter.toggled_tags(true) {
                println!("<{}>", tag.name().unwrap());
                open_tag = tag;
            }

            if iter.ends_tag(Some(&open_tag)) {
                println!("</{}>", open_tag.name().unwrap());
            }
            iter.forward_char();
        }

    }

    pub fn setup(&self) {
        self.set_editable(true);
        self.set_wrap_mode(WrapMode::Word);
        self.set_left_margin(35);
        self.set_right_margin(35);
        self.set_top_margin(24);
        self.set_bottom_margin(24);

        let bold_tag = gtk::TextTag::new(Some("bold"));
        bold_tag.set_weight(600);
        self.buffer().tag_table().add(&bold_tag);

        let italic_tag = gtk::TextTag::new(Some("italics"));
        italic_tag.set_font(Some("Sans italic 12"));
        self.buffer().tag_table().add(&italic_tag);

        //self.add_action(&action_bold);
    }

    pub fn set_name(&self, name: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().name = name.to_string();
    }
 
    pub fn set_file(&self, filename: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().filename = filename.to_string();
    }
    pub fn set_id(&self, id: u32) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().note_id = id;
    }  

    pub fn set_timer(&self, time: u32) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().timer = time;
    }  

    pub fn set_buffstring(&self, buffstring: &String) {
        let vals = Arc::clone(&self.imp().vals);
        vals.lock().unwrap().buffer = buffstring.to_string();
    }  

    pub fn get_file(&self) -> String {
        let vals = Arc::clone(&self.imp().vals);
        let filename = &vals.lock().unwrap().filename; 
        filename.to_string()
    }

    pub fn get_name(&self) -> String {
        let vals = Arc::clone(&self.imp().vals);
        let name = &vals.lock().unwrap().name; 
        name.to_string()
    }

    pub fn get_id(&self) -> u32{
        let vals = Arc::clone(&self.imp().vals);
        let id = vals.lock().unwrap().note_id; id
    }

    pub fn get_vals(&self) -> Arc<Mutex<NoteViewData>> {
        Arc::clone(&self.imp().vals)
    }

}

#[derive(Default)]
pub struct NoteViewData {
    pub name: String,
    pub timer: u32,
    pub buffer: String,
    pub filename: String,
    pub note_id: u32,
}
