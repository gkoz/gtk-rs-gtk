// This file was generated by gir (8cacc34) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use License;
use Widget;
use Window;
use ffi;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AboutDialog(Object<ffi::GtkAboutDialog>): Dialog, Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_about_dialog_get_type(),
    }
}

impl AboutDialog {
    pub fn new() -> AboutDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_about_dialog_new()).downcast_unchecked()
        }
    }

    pub fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(self.to_glib_none().0, section_name.to_glib_none().0, people.to_glib_none().0);
        }
    }

    pub fn get_artists(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_artists(self.to_glib_none().0))
        }
    }

    pub fn get_authors(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_authors(self.to_glib_none().0))
        }
    }

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_comments(self.to_glib_none().0))
        }
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_copyright(self.to_glib_none().0))
        }
    }

    pub fn get_documenters(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_documenters(self.to_glib_none().0))
        }
    }

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_license(self.to_glib_none().0))
        }
    }

    pub fn get_license_type(&self) -> License {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_license_type(self.to_glib_none().0))
        }
    }

    pub fn get_logo(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo(self.to_glib_none().0))
        }
    }

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo_icon_name(self.to_glib_none().0))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_program_name(self.to_glib_none().0))
        }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_translator_credits(self.to_glib_none().0))
        }
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_version(self.to_glib_none().0))
        }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website(self.to_glib_none().0))
        }
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website_label(self.to_glib_none().0))
        }
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_wrap_license(self.to_glib_none().0))
        }
    }

    pub fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(self.to_glib_none().0, artists.to_glib_none().0);
        }
    }

    pub fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(self.to_glib_none().0, authors.to_glib_none().0);
        }
    }

    pub fn set_comments(&self, comments: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.to_glib_none().0, comments.to_glib_none().0);
        }
    }

    pub fn set_copyright(&self, copyright: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.to_glib_none().0, copyright.to_glib_none().0);
        }
    }

    pub fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(self.to_glib_none().0, documenters.to_glib_none().0);
        }
    }

    pub fn set_license(&self, license: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_license(self.to_glib_none().0, license.to_glib_none().0);
        }
    }

    pub fn set_license_type(&self, license_type: License) {
        unsafe {
            ffi::gtk_about_dialog_set_license_type(self.to_glib_none().0, license_type.to_glib());
        }
    }

    pub fn set_logo(&self, logo: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo(self.to_glib_none().0, logo.to_glib_none().0);
        }
    }

    pub fn set_logo_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    pub fn set_program_name(&self, name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn set_translator_credits(&self, translator_credits: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(self.to_glib_none().0, translator_credits.to_glib_none().0);
        }
    }

    pub fn set_version(&self, version: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_version(self.to_glib_none().0, version.to_glib_none().0);
        }
    }

    pub fn set_website(&self, website: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_website(self.to_glib_none().0, website.to_glib_none().0);
        }
    }

    pub fn set_website_label(&self, website_label: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(self.to_glib_none().0, website_label.to_glib_none().0);
        }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(self.to_glib_none().0, wrap_license.to_glib());
        }
    }

    pub fn connect_activate_link<F: Fn(&AboutDialog, &str) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&AboutDialog, &str) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_link_trampoline(this: *mut ffi::GtkAboutDialog, uri: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean {
    callback_guard!();
    let f: &Box_<Fn(&AboutDialog, &str) -> Inhibit + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(uri)).to_glib()
}
