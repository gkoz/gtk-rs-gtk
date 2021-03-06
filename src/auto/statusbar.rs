// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use Buildable;
use Container;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct Statusbar(Object<ffi::GtkStatusbar>): Widget, Container, Box, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_statusbar_get_type(),
    }
}

impl Statusbar {
    pub fn new() -> Statusbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_statusbar_new()).downcast_unchecked()
        }
    }

    pub fn get_context_id(&self, context_description: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_get_context_id(self.to_glib_none().0, context_description.to_glib_none().0)
        }
    }

    pub fn get_message_area(&self) -> Option<Box> {
        unsafe {
            from_glib_none(ffi::gtk_statusbar_get_message_area(self.to_glib_none().0))
        }
    }

    pub fn pop(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_pop(self.to_glib_none().0, context_id);
        }
    }

    pub fn push(&self, context_id: u32, text: &str) -> u32 {
        unsafe {
            ffi::gtk_statusbar_push(self.to_glib_none().0, context_id, text.to_glib_none().0)
        }
    }

    pub fn remove(&self, context_id: u32, message_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove(self.to_glib_none().0, context_id, message_id);
        }
    }

    pub fn remove_all(&self, context_id: u32) {
        unsafe {
            ffi::gtk_statusbar_remove_all(self.to_glib_none().0, context_id);
        }
    }
}
