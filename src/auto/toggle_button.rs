// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ToggleButton(Object<ffi::GtkToggleButton>): Widget, Container, Bin, Button, Actionable, Buildable;

    match fn {
        get_type => || ffi::gtk_toggle_button_get_type(),
    }
}

impl ToggleButton {
    pub fn new() -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> ToggleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toggle_button_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ToggleButtonExt {
    fn get_active(&self) -> bool;

    fn get_inconsistent(&self) -> bool;

    fn get_mode(&self) -> bool;

    fn set_active(&self, is_active: bool);

    fn set_inconsistent(&self, setting: bool);

    fn set_mode(&self, draw_indicator: bool);

    fn toggled(&self);
}

impl<O: IsA<ToggleButton>> ToggleButtonExt for O {
    fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_active(self.to_glib_none().0))
        }
    }

    fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_inconsistent(self.to_glib_none().0))
        }
    }

    fn get_mode(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toggle_button_get_mode(self.to_glib_none().0))
        }
    }

    fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_inconsistent(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_mode(&self, draw_indicator: bool) {
        unsafe {
            ffi::gtk_toggle_button_set_mode(self.to_glib_none().0, draw_indicator.to_glib());
        }
    }

    fn toggled(&self) {
        unsafe {
            ffi::gtk_toggle_button_toggled(self.to_glib_none().0);
        }
    }
}
