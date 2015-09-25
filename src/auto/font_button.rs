// This file was generated by gir (e0b4c3b) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;

pub type FontButton = Object<ffi::GtkFontButton>;

unsafe impl Upcast<Widget> for FontButton { }
unsafe impl Upcast<Container> for FontButton { }
unsafe impl Upcast<Bin> for FontButton { }
unsafe impl Upcast<Button> for FontButton { }
unsafe impl Upcast<Actionable> for FontButton { }
unsafe impl Upcast<Buildable> for FontButton { }

impl FontButton {
    pub fn new() -> FontButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_font(fontname: &str) -> FontButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new_with_font(fontname.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_font_name(self.to_glib_none().0))
        }
    }

    pub fn get_show_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_show_size(self.to_glib_none().0))
        }
    }

    pub fn get_show_style(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_show_style(self.to_glib_none().0))
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_use_font(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_use_font(self.to_glib_none().0))
        }
    }

    pub fn get_use_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_use_size(self.to_glib_none().0))
        }
    }

    pub fn set_font_name(&self, fontname: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_set_font_name(self.to_glib_none().0, fontname.to_glib_none().0))
        }
    }

    pub fn set_show_size(&self, show_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_size(self.to_glib_none().0, show_size.to_glib());
        }
    }

    pub fn set_show_style(&self, show_style: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_style(self.to_glib_none().0, show_style.to_glib());
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_font_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_use_font(&self, use_font: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_font(self.to_glib_none().0, use_font.to_glib());
        }
    }

    pub fn set_use_size(&self, use_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_size(self.to_glib_none().0, use_size.to_glib());
        }
    }

}

impl types::StaticType for FontButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_font_button_get_type()) }
    }
}