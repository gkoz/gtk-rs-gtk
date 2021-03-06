// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use ImageType;
use Menu;
use Orientation;
use Rectangle;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StatusIcon(Object<ffi::GtkStatusIcon>);

    match fn {
        get_type => || ffi::gtk_status_icon_get_type(),
    }
}

impl StatusIcon {
    pub fn new() -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new())
        }
    }

    pub fn new_from_file<T: AsRef<std::path::Path>>(filename: T) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_file(filename.as_ref().to_glib_none().0))
        }
    }

    //pub fn new_from_gicon<T: IsA</*Ignored*/gio::Icon>>(icon: &T) -> StatusIcon {
    //    unsafe { TODO: call ffi::gtk_status_icon_new_from_gicon() }
    //}

    pub fn new_from_icon_name(icon_name: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_icon_name(icon_name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_pixbuf(pixbuf.to_glib_none().0))
        }
    }

    pub fn new_from_stock(stock_id: &str) -> StatusIcon {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_status_icon_new_from_stock(stock_id.to_glib_none().0))
        }
    }

    pub fn get_geometry(&self) -> Option<(gdk::Screen, Rectangle, Orientation)> {
        unsafe {
            let mut screen = ptr::null_mut();
            let mut area = Rectangle::uninitialized();
            let mut orientation = mem::uninitialized();
            let ret = from_glib(ffi::gtk_status_icon_get_geometry(self.to_glib_none().0, &mut screen, area.to_glib_none_mut().0, &mut orientation));
            if ret { Some((from_glib_none(screen), area, orientation)) } else { None }
        }
    }

    //pub fn get_gicon(&self) -> /*Ignored*/Option<gio::Icon> {
    //    unsafe { TODO: call ffi::gtk_status_icon_get_gicon() }
    //}

    pub fn get_has_tooltip(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_has_tooltip(self.to_glib_none().0))
        }
    }

    pub fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_icon_name(self.to_glib_none().0))
        }
    }

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_screen(self.to_glib_none().0))
        }
    }

    pub fn get_size(&self) -> i32 {
        unsafe {
            ffi::gtk_status_icon_get_size(self.to_glib_none().0)
        }
    }

    pub fn get_stock(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_stock(self.to_glib_none().0))
        }
    }

    pub fn get_storage_type(&self) -> ImageType {
        unsafe {
            ffi::gtk_status_icon_get_storage_type(self.to_glib_none().0)
        }
    }

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_status_icon_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_markup(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_markup(self.to_glib_none().0))
        }
    }

    pub fn get_tooltip_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_status_icon_get_tooltip_text(self.to_glib_none().0))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_get_visible(self.to_glib_none().0))
        }
    }

    pub fn get_x11_window_id(&self) -> u32 {
        unsafe {
            ffi::gtk_status_icon_get_x11_window_id(self.to_glib_none().0)
        }
    }

    pub fn is_embedded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_status_icon_is_embedded(self.to_glib_none().0))
        }
    }

    pub fn set_from_file<T: AsRef<std::path::Path>>(&self, filename: T) {
        unsafe {
            ffi::gtk_status_icon_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    //pub fn set_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, icon: &T) {
    //    unsafe { TODO: call ffi::gtk_status_icon_set_from_gicon() }
    //}

    pub fn set_from_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_status_icon_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn set_from_stock(&self, stock_id: &str) {
        unsafe {
            ffi::gtk_status_icon_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0);
        }
    }

    pub fn set_has_tooltip(&self, has_tooltip: bool) {
        unsafe {
            ffi::gtk_status_icon_set_has_tooltip(self.to_glib_none().0, has_tooltip.to_glib());
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_status_icon_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_status_icon_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_status_icon_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_tooltip_markup(&self, markup: Option<&str>) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    pub fn set_tooltip_text(&self, text: &str) {
        unsafe {
            ffi::gtk_status_icon_set_tooltip_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_status_icon_set_visible(self.to_glib_none().0, visible.to_glib());
        }
    }

    pub fn position_menu<T: IsA<Menu>>(menu: &T, x: &mut i32, y: &mut i32, user_data: &StatusIcon) -> bool {
        skip_assert_initialized!();
        unsafe {
            let mut push_in = mem::uninitialized();
            ffi::gtk_status_icon_position_menu(menu.to_glib_none().0, x, y, &mut push_in, user_data.to_glib_none().0);
            from_glib(push_in)
        }
    }
}
