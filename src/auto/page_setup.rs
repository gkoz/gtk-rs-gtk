// This file was generated by gir (30820a7+) from gir-files (11e0e6d)
// DO NOT EDIT

use glib::translate::*;
use glib::types;
use ffi;

use object::*;
use PageOrientation;
use PaperSize;
use Unit;

pub type PageSetup = Object<ffi::GtkPageSetup>;


impl PageSetup {
    pub fn new() -> PageSetup {
        unsafe {
            from_glib_full(ffi::gtk_page_setup_new())
        }
    }

    //pub fn new_from_file(file_name: &str, error: /*Unknown conversion*/Unknown rust type: "Error") -> PageSetup {
    //    unsafe { TODO: call ffi:gtk_page_setup_new_from_file() }
    //}

    //pub fn new_from_key_file(key_file: /*Unknown conversion*/Unknown rust type: "KeyFile", group_name: Option<&str>, error: /*Unknown conversion*/Unknown rust type: "Error") -> PageSetup {
    //    unsafe { TODO: call ffi:gtk_page_setup_new_from_key_file() }
    //}

    pub fn copy(&self) -> Option<PageSetup> {
        unsafe {
            from_glib_full(ffi::gtk_page_setup_copy(self.to_glib_none().0))
        }
    }

    pub fn get_bottom_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_bottom_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_left_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_left_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_orientation(&self) -> PageOrientation {
        unsafe {
            ffi::gtk_page_setup_get_orientation(self.to_glib_none().0)
        }
    }

    pub fn get_page_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_page_height(self.to_glib_none().0, unit)
        }
    }

    pub fn get_page_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_page_width(self.to_glib_none().0, unit)
        }
    }

    pub fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_paper_height(self.to_glib_none().0, unit)
        }
    }

    pub fn get_paper_size(&self) -> PaperSize {
        unsafe { from_glib_none(ffi::gtk_page_setup_get_paper_size(self.to_glib_none().0)) }
    }

    pub fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_paper_width(self.to_glib_none().0, unit)
        }
    }

    pub fn get_right_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_right_margin(self.to_glib_none().0, unit)
        }
    }

    pub fn get_top_margin(&self, unit: Unit) -> f64 {
        unsafe {
            ffi::gtk_page_setup_get_top_margin(self.to_glib_none().0, unit)
        }
    }

    //pub fn load_file(&self, file_name: &str, error: /*Unknown conversion*/Unknown rust type: "Error") -> bool {
    //    unsafe { TODO: call ffi:gtk_page_setup_load_file() }
    //}

    //pub fn load_key_file(&self, key_file: /*Unknown conversion*/Unknown rust type: "KeyFile", group_name: Option<&str>, error: /*Unknown conversion*/Unknown rust type: "Error") -> bool {
    //    unsafe { TODO: call ffi:gtk_page_setup_load_key_file() }
    //}

    pub fn set_bottom_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_bottom_margin(self.to_glib_none().0, margin, unit);
        }
    }

    pub fn set_left_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_left_margin(self.to_glib_none().0, margin, unit);
        }
    }

    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            ffi::gtk_page_setup_set_orientation(self.to_glib_none().0, orientation);
        }
    }

    //pub fn set_paper_size(&self, size: /*Unknown conversion*/Unknown rust type: "PaperSize") {
    //    unsafe { TODO: call ffi:gtk_page_setup_set_paper_size() }
    //}

    //pub fn set_paper_size_and_default_margins(&self, size: /*Unknown conversion*/Unknown rust type: "PaperSize") {
    //    unsafe { TODO: call ffi:gtk_page_setup_set_paper_size_and_default_margins() }
    //}

    pub fn set_right_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_right_margin(self.to_glib_none().0, margin, unit);
        }
    }

    pub fn set_top_margin(&self, margin: f64, unit: Unit) {
        unsafe {
            ffi::gtk_page_setup_set_top_margin(self.to_glib_none().0, margin, unit);
        }
    }

    //pub fn to_file(&self, file_name: &str, error: /*Unknown conversion*/Unknown rust type: "Error") -> bool {
    //    unsafe { TODO: call ffi:gtk_page_setup_to_file() }
    //}

    //pub fn to_key_file(&self, key_file: /*Unknown conversion*/Unknown rust type: "KeyFile", group_name: &str) {
    //    unsafe { TODO: call ffi:gtk_page_setup_to_key_file() }
    //}

}

impl types::StaticType for PageSetup {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_page_setup_get_type()) }
    }
}
