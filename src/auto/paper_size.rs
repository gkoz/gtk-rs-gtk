// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::types;
use ffi;

use Unit;

glib_wrapper! {
    ///Paper size
    pub struct PaperSize(Boxed<ffi::GtkPaperSize>);

    match fn {
        copy => |ptr| ffi::gtk_paper_size_copy(ptr as *mut ffi::GtkPaperSize),
        free => |ptr| ffi::gtk_paper_size_free(ptr),
    }
}

impl PaperSize {
    pub fn new(name: &str) -> PaperSize {
        unsafe { from_glib_none(ffi::gtk_paper_size_new(name.to_glib_none().0)) }
    }

    pub fn new_from_ppd(ppd_name: &str, ppd_display_name: &str, width: f64, height: f64) -> PaperSize {
        unsafe { from_glib_none(ffi::gtk_paper_size_new_from_ppd(ppd_name.to_glib_none().0,
                                             ppd_display_name.to_glib_none().0,
                                             width, height)) }
    }

    pub fn new_custom(name: &str, display_name: &str, width: f64, height: f64, unit: Unit) -> PaperSize {
        unsafe { from_glib_none(ffi::gtk_paper_size_new_custom(name.to_glib_none().0,
                                           display_name.to_glib_none().0,
                                           width, height, unit)) }
    }

    pub fn is_equal(&self, other: &PaperSize) -> bool {
        unsafe { from_glib(ffi::gtk_paper_size_is_equal(self.to_glib_none().0, other.to_glib_none().0)) }
    }

    //pub fn get_paper_sizes(include_custom: bool) -> glib::List<Box<PaperSize>> {
    // TODO: call ffi::gtk_paper_size_get_paper_sizes

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_paper_size_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_display_name_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_paper_size_get_display_name(self.to_glib_none().0))
        }
    }

    pub fn get_ppd_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_paper_size_get_ppd_name(self.to_glib_none().0))
        }
    }

    pub fn get_width(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_width(self.to_glib_none().0, unit) }
    }

    pub fn get_height(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_height(self.to_glib_none().0, unit) }
    }

    pub fn is_custom(&self) -> bool {
        unsafe { from_glib(ffi::gtk_paper_size_is_custom(self.to_glib_none().0)) }
    }

    pub fn set_size(&self, width: f64, height: f64, unit: Unit) {
        unsafe { ffi::gtk_paper_size_set_size(self.to_glib_none().0, width, height, unit) }
    }

    pub fn get_default_top_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_top_margin(self.to_glib_none().0, unit) }
    }

    pub fn get_default_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_bottom_margin(self.to_glib_none().0, unit) }
    }

    pub fn get_default_left_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(self.to_glib_none().0, unit) }
    }

    pub fn get_default_right_margin(&self, unit: Unit) -> f64 {
        unsafe { ffi::gtk_paper_size_get_default_left_margin(self.to_glib_none().0, unit) }
    }

    pub fn get_default() -> Option<String> {
        unsafe {
            from_glib_none(
                ffi::gtk_paper_size_get_default())
        }
    }
}

impl types::StaticType for PaperSize {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_paper_size_get_type()) }
    }
}
